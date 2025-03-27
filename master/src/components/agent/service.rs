use entity::agent;
use helpers::{
  jwt::{self, sign},
  time::utc_now,
};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{Value, json};
use tracing::error;

use crate::{app::AppState, components::user::model::is_admin, error::AppError};

use super::model::{AgentQueryBy, get_agent, get_agent_heartbeat, has_agent};

pub async fn register_agent(
  state: &AppState,
  user_id: String,
  hostname: String,
  ip_address: String,
  storage_path: String,
  available_space: u32,
  status: String,
) -> Result<Value, AppError> {
  if !is_admin(user_id, &state.db).await? {
    return Err(AppError::Forbidden);
  }
  if has_agent(AgentQueryBy::Hostname(hostname.clone()), &state.db).await?
    || has_agent(AgentQueryBy::IpAddress(ip_address.clone()), &state.db).await?
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
    status: Set(status),
    token: Set(token),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  let result = active_agent.insert(&state.db).await?;
  Ok(json!(result))
}

pub async fn get_agent_status(state: &AppState, agent_id: u32) -> Result<Value, AppError> {
  let agent = get_agent(agent_id, &state.db).await?;
  let data = get_agent_heartbeat(agent.ip_address).await.map_err(|err| {
    error!("{}", err);
    AppError::RpcCallError
  })?;

  Ok(json!({
    "cpu_cores" : data.data.cpu_cores,
    "cpu_usage" : data.data.cpu_usage,
    "total_memory": data.data.total_memory,
    "free_memory": data.data.free_memory,
    "memory_usage": data.data.memory_usage,
  }))
}

pub async fn refresh_agent_token(
  state: &AppState,
  user_id: String,
  agent_id: u32,
) -> Result<Value, AppError> {
  if !is_admin(user_id, &state.db).await? {
    return Err(AppError::Forbidden);
  }
  if !has_agent(AgentQueryBy::Id(agent_id.clone()), &state.db).await? {
    return Err(AppError::AgentNotFound);
  }

  let token = sign::<String>(
    agent_id.to_string(),
    &state.register_agent_key,
    state.register_agent_key_expire,
  )?;

  agent::ActiveModel {
    id: Set(agent_id),
    token: Set(token.clone()),
    updated_at: Set(Some(utc_now())),
    ..Default::default()
  }
  .update(&state.db)
  .await?;

  Ok(json!({
    "token": token
  }))
}
