use std::time::Duration;

use serde_json::json;

use crate::{error::AppError, response::Response};

pub struct MasterRpc {
  agent_id: u32,
  agent_token: String,
  master_url: String,
  api_client: reqwest::Client,
}

impl MasterRpc {
  pub fn new(master_url: String, agent_token: String, agent_id: u32) -> Self {
    Self {
      agent_id,
      agent_token,
      master_url,
      api_client: reqwest::Client::new(),
    }
  }
  pub async fn update_deployment_status(&self) -> Result<(), AppError> {
    let resp = self
      .api_client
      .post(format!("{}/api/deployment/status", self.master_url))
      .json(&json!({
        "agent_id": self.agent_id,
        "agent_token": self.agent_token.to_string(),
        "deployment_id": 1,
        "status": "reviewing"
      }))
      .timeout(Duration::from_secs(3))
      .send()
      .await?;
    resp.json::<Response<()>>().await?;
    Ok(())
  }
}
