use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadRequest {
  pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskPublishRequest {
  pub site_id: String,
  pub deployment_id: u32,
  pub bandwidth: String,
  pub bind_domain: Option<String>,
  pub preview_domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRevokeRequest {
  pub site_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeartbeatResponse {
  pub cpu_cores: usize,
  pub cpu_usage: f32,
  pub total_memory: u64,
  pub free_memory: u64,
  pub memory_usage: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InitUploadResponse {
  pub upload_token: String,
}
