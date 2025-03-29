use entity::deployment;
use helpers::{jwt, time::utc_now};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{Value, json};

use crate::{app::AppState, components::agent::model::get_agent, error::AppError};

use super::model::{DeploymentQueryBy, get_deployment, has_deployment};

pub async fn get_deployment_info(state: &AppState, deployment_id: u32) -> Result<Value, AppError> {
  let deployment = get_deployment(DeploymentQueryBy::Id(deployment_id), &state.db).await?;
  Ok(json!(deployment))
}

pub async fn update_deployment_status(
  state: &AppState,
  agent_id: u32,
  agent_token: String,
  deployment_id: u32,
  status: String,
) -> Result<Value, AppError> {
  let agent = get_agent(agent_id, &state.db).await?;
  if agent.token != agent_token {
    return Err(AppError::AgentAuthFailed);
  }
  if !has_deployment(DeploymentQueryBy::Id(deployment_id), &state.db).await? {
    return Err(AppError::DeploymentNotFound);
  }
  jwt::verify::<String>(&agent_token, &state.register_agent_key)?;
  let deployment = deployment::ActiveModel {
    id: Set(deployment_id),
    status: Set(status),
    execution_time: Set(utc_now()),
    ..Default::default()
  }
  .update(&state.db)
  .await?;
  Ok(json!(deployment))
}
