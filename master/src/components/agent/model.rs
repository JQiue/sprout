use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterAgentBody {
  pub hostname: String,
  pub ip_address: String,
  pub storage_path: String,
  pub available_space: u32,
}
