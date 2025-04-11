use std::{net::Ipv4Addr, str::FromStr};

use entity::{
  agent::{self, AgentStatus},
  deployment::DeploymentStatus,
};
use helpers::{jwt, time::utc_now};
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde_json::{Value, json};

use crate::{app::AppState, error::AppError};

pub async fn register_agent(
  state: &AppState,
  user_id: String,
  hostname: String,
  ip_address: String,
  storage_path: String,
  available_space: u32,
) -> Result<Value, AppError> {
  if !state.repo.user().is_admin_user(&user_id).await? {
    return Err(AppError::Forbidden);
  }
  if state
    .repo
    .agent()
    .has_agent_by_ip(ip_address.clone())
    .await?
  {
    return Err(AppError::AgentExist);
  }

  let token = jwt::sign::<String>(
    hostname.to_owned(),
    &state.register_agent_key,
    state.register_agent_key_expire,
  )?;
  let active_agent = agent::ActiveModel {
    hostname: Set(hostname),
    ip_address: Set(ip_address),
    storage_path: Set(storage_path),
    available_space: Set(available_space),
    status: Set(AgentStatus::Online),
    token: Set(token),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  let result = active_agent.insert(&state.db).await?;
  Ok(json!(result))
}

pub async fn get_agent_status(state: &AppState, agent_id: u32) -> Result<Value, AppError> {
  if let Some(agent) = state.repo.agent().get_agent(agent_id).await? {
    let data = rpc::AgentRpc::new()
      .get_agent_heartbeat(&agent.ip_address)
      .await?;
    let mut active_agent = agent.into_active_model();
    active_agent.last_heartbeat = Set(Some(utc_now()));
    state.repo.agent().update_agent(active_agent).await?;
    Ok(json!({
      "cpu_cores" : data.cpu_cores,
      "cpu_usage" : data.cpu_usage,
      "total_memory": data.total_memory,
      "free_memory": data.free_memory,
      "memory_usage": data.memory_usage,
    }))
  } else {
    Err(AppError::AgentNotFound)
  }
}

pub async fn refresh_agent_token(
  state: &AppState,
  user_id: String,
  agent_id: u32,
) -> Result<Value, AppError> {
  if !state.repo.user().is_admin_user(&user_id).await? {
    return Err(AppError::Forbidden);
  }
  if !state.repo.agent().has_agent_by_id(agent_id).await? {
    return Err(AppError::AgentNotFound);
  }

  let token = jwt::sign::<String>(
    agent_id.to_string(),
    &state.register_agent_key,
    state.register_agent_key_expire,
  )?;
  let agent = state
    .repo
    .agent()
    .update_agent(agent::ActiveModel {
      id: Set(agent_id),
      token: Set(token.clone()),
      updated_at: Set(Some(utc_now())),
      ..Default::default()
    })
    .await?;
  Ok(json!({
    "token": agent.token
  }))
}

pub async fn assign_task(
  state: &AppState,
  r#type: String,
  site_id: String,
  deployment_id: u32,
) -> Result<Value, AppError> {
  let site = if let Some(site) = state.repo.site().get_site_by_id(&site_id).await? {
    site
  } else {
    return Err(AppError::SiteNotFound);
  };
  let deployment = state
    .repo
    .deployment()
    .get_deployment(deployment_id)
    .await?
    .ok_or(AppError::DeploymentNotFound)?;
  let agent = state
    .repo
    .agent()
    .get_agent(deployment.agent_id)
    .await?
    .ok_or(AppError::AgentNotFound)?;
  let preview_domain = format!("preview_{}.jinqiu.wang", site_id);
  state
    .cloudflare_rpc
    .create_a_record(&preview_domain, Ipv4Addr::from_str(&agent.ip_address)?)
    .await;
  if r#type == "publish" {
    state
      .agent_rpc
      .task_publish(
        site_id,
        deployment_id,
        agent.ip_address,
        site.bandwidth.to_string(),
        site.domain,
        preview_domain.clone(),
      )
      .await?;
    let preview_url = "http://".to_string() + &preview_domain;
    let mut active_deployment = deployment.into_active_model();
    active_deployment.status = Set(DeploymentStatus::Published);
    active_deployment.deploy_preview_url = Set(Some(preview_url.clone()));
    state
      .repo
      .deployment()
      .update_deployment(active_deployment)
      .await?;
    return Ok(json!({
      "preview_url": preview_url
    }));
  } else if r#type == "revoke" {
    state
      .agent_rpc
      .task_revoke(site_id, &agent.ip_address)
      .await?;
  }
  Ok(Value::Null)
}
