use helpers::{jwt::sign, time::utc_now};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{json, Value};
use tracing::error;

use crate::{
  app::AppState, components::user::model::is_admin, entities::agent, response::StatusCode,
};

use super::model::{
  get_agent, get_agent_heartbeat, has_agent, AgentQueryBy, AgentStatus, GetAgentHeartBeatJson,
};

pub async fn register_agent(
  state: &AppState,
  user_id: String,
  hostname: String,
  ip_address: String,
  storage_path: String,
  available_space: u32,
  status: AgentStatus,
) -> Result<Value, StatusCode> {
  if !is_admin(user_id, &state.db).await? {
    return Err(StatusCode::Forbidden);
  }
  if has_agent(AgentQueryBy::Hostname(hostname.clone()), &state.db).await?
    || has_agent(AgentQueryBy::IpAddress(ip_address.clone()), &state.db).await?
  {
    return Err(StatusCode::AgentExist);
  }

  let token = sign::<String>(
    hostname.to_owned(),
    state.register_agent_key.clone(),
    state.register_agent_key_expire,
  )
  .map_err(|_| StatusCode::ServerError)?;

  let model = agent::ActiveModel {
    hostname: Set(hostname),
    ip_address: Set(ip_address),
    storage_path: Set(storage_path),
    available_space: Set(available_space),
    status: Set(status),
    token: Set(token),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  let result = model
    .insert(&state.db)
    .await
    .map_err(|_| StatusCode::DbError)?;
  Ok(json!(result))
}

pub async fn get_agent_status(
  state: &AppState,
  agent_id: i32,
) -> Result<GetAgentHeartBeatJson, StatusCode> {
  let agent = get_agent(agent_id, &state.db).await?;
  let res = get_agent_heartbeat(agent.ip_address).await.map_err(|err| {
    error!("{}", err);
    StatusCode::SendAgentRequestError
  });
  res
}

pub async fn refresh_agent_token(
  state: &AppState,
  user_id: String,
  agent_id: i32,
) -> Result<Value, StatusCode> {
  if !is_admin(user_id, &state.db).await? {
    return Err(StatusCode::Forbidden);
  }
  if !has_agent(AgentQueryBy::Id(agent_id.clone()), &state.db).await? {
    return Err(StatusCode::AgentNotFound);
  }

  let token = sign::<String>(
    agent_id.to_string(),
    state.register_agent_key.to_owned(),
    state.register_agent_key_expire,
  )
  .map_err(|_| StatusCode::ServerError)?;

  agent::ActiveModel {
    id: Set(agent_id),
    token: Set(token.clone()),
    updated_at: Set(Some(utc_now())),
    ..Default::default()
  }
  .update(&state.db)
  .await
  .map_err(|_| StatusCode::DbError)?;

  Ok(json!({
    "token": token
  }))
}
