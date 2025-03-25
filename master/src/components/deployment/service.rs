use helpers::time::utc_now;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde_json::{json, Value};

use crate::{
  app::AppState, components::agent::model::get_agent, entities::deployment, response::StatusCode,
};

use super::model::{get_deployment, has_deployment, DeploymentQueryBy, DeploymentStatus};

pub async fn get_deployment_info(
  state: &AppState,
  deployment_id: i32,
) -> Result<Value, StatusCode> {
  let deployment = get_deployment(DeploymentQueryBy::Id(deployment_id), &state.db).await?;
  Ok(json!(deployment))
}

pub async fn update_deployment_status(
  agent_id: i32,
  agent_token: String,
  deployment_id: i32,
  status: DeploymentStatus,
  db: &DatabaseConnection,
) -> Result<Value, StatusCode> {
  let agent = get_agent(agent_id, db).await?;
  if agent.token != agent_token {
    return Err(StatusCode::AgentAuthFailed);
  }
  if !has_deployment(super::model::DeploymentQueryBy::Id(deployment_id), db).await? {
    return Err(StatusCode::DeploymentNotFound);
  }
  helpers::jwt::verify::<String>(agent_token, "agent_key".to_owned())
    .map_err(|_| StatusCode::AuthFailed)?;
  let model = deployment::ActiveModel {
    id: Set(deployment_id),
    status: Set(status),
    execution_time: Set(utc_now()),
    ..Default::default()
  }
  .update(db)
  .await
  .map_err(|_| StatusCode::DbError)?;
  Ok(json!(model))
}
