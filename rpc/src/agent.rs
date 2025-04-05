use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse<T> {
  pub code: i32,
  pub msg: String,
  pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadData {
  pub upload_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AgentHeartbeat {
  pub cpu_cores: i8,
  pub cpu_usage: f32,
  pub total_memory: i32,
  pub free_memory: i32,
  pub memory_usage: f32,
}

pub struct Rpc {
  api_client: reqwest::Client,
}

impl Rpc {
  pub fn new() -> Self {
    Self {
      api_client: reqwest::Client::new(),
    }
  }

  pub async fn get_agent_heartbeat(&self, agent_ip: String) -> Result<AgentHeartbeat, AppError> {
    let resp = self
      .api_client
      .get(format!("http://{agent_ip}:5001/api/heartbeat"))
      .send()
      .await?;
    let data = resp.json::<RpcResponse<AgentHeartbeat>>().await?;
    Ok(data.data)
  }

  pub async fn init_upload_session(
    &self,
    agent_ip: &str,
    site_id: &str,
    deploy_id: u32,
  ) -> Result<InitUploadData, AppError> {
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/upload/init", agent_ip))
      .timeout(Duration::from_secs(3))
      .json(&json!({
        "site_id": site_id,
        "deploy_id": deploy_id
      }))
      .send()
      .await?;
    let data = resp.json::<RpcResponse<InitUploadData>>().await?;
    debug!("Response body: {:?}", data);
    Ok(data.data)
  }
}
