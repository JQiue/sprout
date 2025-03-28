use std::{
  fs,
  path::{Path, PathBuf},
};

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

#[derive(Deserialize)]
pub struct GetCasualTokenData {
  pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeployData {
  pub upload_url: String,
  pub upload_token: String,
  pub site_id: String,
  pub agent_id: u32,
  pub deploy_id: u32,
}

pub struct Rpc {
  master_url: String,
  api_client: reqwest::Client,
}

impl Rpc {
  pub fn new(master_url: String) -> Self {
    Self {
      master_url,
      api_client: reqwest::Client::new(),
    }
  }

  pub async fn get_casual_token(&self) -> std::string::String {
    let resp = self
      .api_client
      .get(format!("{}/api/user/casual", self.master_url))
      .send()
      .await
      .unwrap();

    if resp.status() == 200 {}
    println!("{:?}", resp);
    let data = resp.json::<Response<GetCasualTokenData>>().await.unwrap();
    data.data.unwrap().token
  }

  fn login() {}

  pub async fn upload(&self, deploy_data: DeployData, path: PathBuf) {
    println!("{:?}", path);
    let path_buf = path.clone();
    let file_name = path
      .file_name()
      .and_then(|n| n.to_str())
      .map(|s| s.to_string())
      .unwrap_or_else(|| "unknown".to_string());
    let part = reqwest::multipart::Part::file(path_buf)
      .await
      .unwrap()
      .file_name(file_name)
      .mime_str("application/octet-stream")
      .unwrap();
    let mut form = Form::new().part("dist", part);
    form = form.part("upload_token", Part::text(deploy_data.upload_token));
    let resp = self
      .api_client
      .post(format!(
        "http://{}:5001/api/upload/file",
        deploy_data.upload_url
      ))
      .multipart(form)
      .send()
      .await
      .unwrap();
    println!("{:?}", resp);
  }

  pub async fn create_site(&self, token: String) -> DeployData {
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
    let data = resp.json::<Response<DeployData>>().await.unwrap();
    println!("{:?}", data);
    data.data.unwrap()
  }
  // pub async fn update_deployment_status(&self) {
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
  //     .await;

  //   let data = resp.json::<Response<()>>().await?;
  // }
}
