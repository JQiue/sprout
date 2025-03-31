use entity::deployment::DeploymentStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeploymentStatusBody {
  pub agent_id: u32,
  pub agent_token: String,
  pub deployment_id: u32,
  pub status: DeploymentStatus,
}
