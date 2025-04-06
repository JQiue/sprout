use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::trace;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
  Pending,
  Uploading,
  Reviewing,
  Published,
  Failed,
}

#[derive(Deserialize)]
pub struct GetCasualTokenData {
  pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginData {
  token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateSiteData {
  pub site_id: String,
  pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateDeploymentData {
  pub site_id: String,
  pub deployment_id: u32,
  pub upload_url: String,
  pub upload_token: String,
  pub agent_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeploySiteData {
  pub domian: String,
}

pub struct Rpc {
  master_url: String,
  api_client: reqwest::Client,
}

impl Rpc {
  pub fn new() -> Self {
    Self {
      master_url: "http://127.0.0.1:3000".to_string(),
      api_client: reqwest::Client::new(),
    }
  }

  pub async fn login(&self, username: String, password: String) -> std::string::String {
    let resp = self
      .api_client
      .post(format!("{}/api/user/token", self.master_url))
      .json(&json!({
        "email": username,
        "password": password
      }))
      .send()
      .await
      .unwrap();
    trace!(">>> resp {:?}", resp);
    let data = resp.json::<Response<LoginData>>().await.unwrap();
    trace!(">>> data {:?}", data);
    if data.code != 0 {
      panic!("login failed: {}", data.msg);
    }
    data.data.unwrap().token
  }

  pub async fn get_casual_token(&self) -> std::string::String {
    let resp = self
      .api_client
      .get(format!("{}/api/user/casual", self.master_url))
      .send()
      .await
      .unwrap();
    trace!("{:?}", resp);
    let data = resp.json::<Response<GetCasualTokenData>>().await.unwrap();
    data.data.unwrap().token
  }

  pub async fn create_site(&self, token: &str) -> CreateSiteData {
    let resp = self
      .api_client
      .post(format!("{}/api/site", self.master_url))
      .bearer_auth(token)
      .json(&json!({
        "site_name": "casual_site"
      }))
      .send()
      .await
      .unwrap();
    let data = resp.json::<Response<CreateSiteData>>().await.unwrap();
    trace!("{:?}", data);
    data.data.unwrap()
  }

  pub async fn create_deployment(&self, site_id: &str, token: &str) -> CreateDeploymentData {
    let resp = self
      .api_client
      .post(format!("{}/api/deployment", self.master_url))
      .bearer_auth(token)
      .json(&json!({
        "site_id": site_id
      }))
      .send()
      .await
      .unwrap();
    let data = resp.json::<Response<CreateDeploymentData>>().await.unwrap();
    trace!("{:?}", data);
    data.data.unwrap()
  }

  pub async fn update_deployment_status(
    &self,
    agent_token: String,
    deployment_id: u32,
    status: DeploymentStatus,
  ) -> Result<(), AppError> {
    let resp = self
      .api_client
      .post(format!("{}/api/deployment/status", self.master_url))
      .json(&json!({
        "agent_token": agent_token,
        "deployment_id": deployment_id,
        "status": status
      }))
      .timeout(Duration::from_secs(3))
      .send()
      .await?;
    resp.json::<Response<()>>().await?;
    Ok(())
  }

  pub async fn deploy_site(&self, token: &str, site_id: &str) -> DeploySiteData {
    let resp = self
      .api_client
      .post(format!("{}/api/deployment", self.master_url))
      .bearer_auth(token)
      .json(&json!({
        "site_id": site_id
      }))
      .send()
      .await
      .unwrap();
    let data = resp.json::<Response<DeploySiteData>>().await.unwrap();
    trace!("{:?}", data);
    data.data.unwrap()
  }
}
