use crate::{app::AppState, error::AppError, rpc::AgentRpc};
use entity::{
  deployment::{self, DeploymentStatus},
  site,
};
use helpers::{
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use sea_orm::{IntoActiveModel, Set};
use serde_json::{Value, json};

/// Creates a new site for a user.
///
/// # Arguments
///
/// * `state` - A reference to the application state.
/// * `user_id` - The unique identifier of the user creating the site.
/// * `site_name` - The name of the site.
/// * `site_type` - The type of site being created.
/// * `repo_url` - An optional URL to the repository associated with the site.
///
/// # Returns
///
/// A `Result` containing a `Value` with the site creation status on success,
/// or a `String` error message on failure.
pub async fn create_site(
  state: &AppState,
  user_id: String,
  site_name: String,
) -> Result<Value, AppError> {
  if let Some(agent) = state.repo.agent().get_avaliable_agent().await? {
    let site = state
      .repo
      .site()
      .create_site(site::ActiveModel {
        site_id: Set(nanoid(&Alphabet::LOWER, 25)),
        name: Set(site_name),
        user_id: Set(user_id),
        created_at: Set(utc_now()),
        ..Default::default()
      })
      .await?;
    let deployment = state
      .repo
      .deployment()
      .create_deployment(deployment::ActiveModel {
        status: Set(DeploymentStatus::Pending),
        agent_id: Set(agent.id),
        site_id: Set(site.clone().site_id),
        execution_time: Set(utc_now()),
        created_at: Set(utc_now()),
        ..Default::default()
      })
      .await?;
    let agent_rpc = AgentRpc::new();
    let init_response = agent_rpc
      .init_upload_session(&agent.ip_address, deployment.clone())
      .await?;
    let mut active_deployment = deployment.into_active_model();
    active_deployment.status = Set(DeploymentStatus::Uploading);
    active_deployment.upload_token = Set(Some(init_response.upload_token.clone()));
    let deployment = state
      .repo
      .deployment()
      .update_deployment(active_deployment)
      .await?;
    Ok(json!({
        "upload_url": agent.ip_address,
        "upload_token": init_response.upload_token,
        "site_id": site.site_id,
        "agent_id": deployment.agent_id,
        "deploy_id": deployment.id,
    }))
  } else {
    Err(AppError::AgentNotFound)
  }
}
