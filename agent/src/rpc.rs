use serde_json::json;

use crate::{error::AppError, response::Response};

struct Rpc {
  agent_id: String,
  agent_token: String,
  master_url: String,
  api_client: reqwest::Client,
}

impl Rpc {
  fn new(master_url: String, agent_token: String, agent_id: String) -> Self {
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
      .send()
      .await?;

    let data = resp.json::<Response<()>>().await?;
    Ok(())
  }
}
