use std::time::Duration;

use entity::deployment;
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

pub struct AgentRpc {
  api_client: reqwest::Client,
}

impl AgentRpc {
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
    deployment: deployment::Model,
  ) -> Result<InitUploadData, AppError> {
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/upload/init", agent_ip))
      .timeout(Duration::from_secs(3))
      .json(&json!({
        "site_id": deployment.site_id,
        "deploy_id": deployment.id
      }))
      .send()
      .await?;
    let data = resp.json::<RpcResponse<InitUploadData>>().await?;
    debug!("Response body: {:?}", data);
    Ok(data.data)
  }
}
