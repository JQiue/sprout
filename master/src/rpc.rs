use std::time::Duration;

use entity::deployment;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};

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

pub struct AgentRpc {
  api_client: reqwest::Client,
}

impl AgentRpc {
  pub fn new() -> Self {
    Self {
      api_client: reqwest::Client::new(),
    }
  }

  pub async fn init_upload_session(
    &self,
    agent_ip: &str,
    deployment: deployment::Model,
  ) -> Result<InitUploadData, AppError> {
    let resp = reqwest::Client::new()
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

  // pub async fn update_deployment_status(&self) -> Result<(), AppError> {
  //   let resp = self
  //     .api_client
  //     .post(format!("{}/api/deployment/status", self.master_url))
  //     .json(&json!({
  //       "agent_id": self.agent_id,
  //       "agent_token": self.agent_token.to_string(),
  //       "deployment_id": 1,
  //       "status": "reviewing"
  //     }))
  //     .send()
  //     .await?;

  //   let data = resp.json::<Response<()>>().await?;
  //   Ok(())
  // }
}
