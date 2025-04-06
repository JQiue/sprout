use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterAgentBody {
  pub hostname: String,
  pub ip_address: String,
  pub storage_path: String,
  pub available_space: u32,
}

#[derive(Serialize, Deserialize)]
pub struct AssignTaskBody {
  pub r#type: String,
  pub site_id: String,
  pub deployment_id: u32,
}
