use std::{path::PathBuf, time::Duration};

use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, trace};

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

#[derive(Debug, Deserialize, Clone)]
pub struct UploadData {
  domian: String,
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

  pub async fn upload(
    &self,
    upload_url: String,
    upload_token: String,
    deployment_id: u32,
    path: PathBuf,
  ) -> std::string::String {
    println!(">>> {:?}", path);
    let path_buf = path.clone();
    let file_name = path
      .file_name()
      .and_then(|n| n.to_str())
      .map(|s| s.to_string())
      .unwrap_or_else(|| "unknown".to_string());
    let part = Part::file(path_buf)
      .await
      .unwrap()
      .file_name(file_name)
      .mime_str("application/octet-stream")
      .unwrap();
    let mut form = Form::new().part("dist", part);
    form = form.part("upload_token", Part::text(upload_token));
    form = form.part("deployment_id", Part::text(deployment_id.to_string()));
    trace!(">>> upload file");
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/upload/file", upload_url))
      .multipart(form)
      .send()
      .await
      .unwrap();
    trace!(">>> {:?}", resp);
    let data = resp.json::<RpcResponse<UploadData>>().await.unwrap();
    trace!(">>> {:?}", data);
    data.data.domian
  }
}
