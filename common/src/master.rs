use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Clone)]
pub struct UserRegisterRequest {
  #[validate(length(min = 2, max = 12))]
  pub nickname: String,
  #[validate(email)]
  pub email: String,
  #[validate(length(min = 8, max = 16))]
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssignTaskRequest {
  pub r#type: String,
  pub site_id: String,
  pub deployment_id: u32,
  pub bind_domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeploymentRequest {
  pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeploymentResponse {
  pub deploy_url: String,
  pub deploy_token: String,
  pub site_id: String,
  pub agent_id: u32,
  pub deployment_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSitesResponse {
  pub sites: Vec<entity::site::Model>,
}
