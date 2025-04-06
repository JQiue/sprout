use entity::agent::{self, AgentStatus};
use helpers::{jwt, time::utc_now};
use sea_orm::{ActiveModelTrait, Set};
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
    let data = rpc::Agent::Rpc::new()
      .get_agent_heartbeat(agent.ip_address)
      .await?;
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

  if r#type == "publish" {
    let domian = rpc::Agent::Rpc::new()
      .task_publish(&site_id, deployment_id, &agent.ip_address)
      .await
      .domian;
    return Ok(json!({
      "domian": domian
    }));
  } else if r#type == "revoke" {
    let domian = rpc::Agent::Rpc::new()
      .task_revoke(&site_id, deployment_id, &agent.ip_address)
      .await
      .domian;
  }

  Ok(Value::Null)
}
