pub mod error;

use core::panic;
use std::fmt::Debug;
use std::{path::PathBuf, time::Duration};

use common::agent::{
  HeartbeatResponse, InitUploadRequest, InitUploadResponse, TaskPublishRequest, TaskRevokeRequest,
};
use common::master::AssignTaskRequest;
use log::{error, trace};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse<T> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadData {
  pub upload_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskPublishData {
  pub preview_url: String,
}

#[derive(Debug, Clone)]
pub struct AgentRpc {
  api_client: reqwest::Client,
}

impl AgentRpc {
  pub fn new() -> Self {
    Self {
      api_client: reqwest::Client::new(),
    }
  }

  pub async fn get_agent_heartbeat(&self, agent_ip: &str) -> Result<HeartbeatResponse, AppError> {
    let resp = self
      .api_client
      .get(format!("http://{agent_ip}:5001/api/heartbeat"))
      .timeout(Duration::from_secs(3))
      .send()
      .await?;
    let data = resp.json::<RpcResponse<HeartbeatResponse>>().await?;
    if data.code != 0 {
      error!("{}", data.msg);
      Err(AppError::RpcCallError)
    } else {
      Ok(data.data.unwrap())
    }
  }

  pub async fn init_upload_session(
    &self,
    agent_ip: &str,
    site_id: String,
  ) -> Result<InitUploadResponse, AppError> {
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/upload/init", agent_ip))
      .timeout(Duration::from_secs(3))
      .json(&InitUploadRequest { site_id })
      .send()
      .await?;
    let data = resp.json::<RpcResponse<InitUploadResponse>>().await?;
    if data.code == 0 {
      Ok(data.data.unwrap())
    } else {
      error!("{}", data.msg);
      Err(AppError::RpcCallError)
    }
  }

  pub async fn upload_file(
    &self,
    upload_url: String,
    upload_token: String,
    deployment_id: u32,
    path: PathBuf,
  ) -> Result<bool, AppError> {
    trace!(">>> {:?}", path);
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
      .await?;
    let data = resp.json::<RpcResponse<()>>().await?;
    if data.code != 0 {
      error!("{}", data.msg);
      Ok(false)
    } else {
      Ok(true)
    }
  }

  pub async fn task_publish(
    &self,
    site_id: String,
    deployment_id: u32,
    ip_address: String,
    bandwidth: String,
    bind_domain: Option<String>,
    preview_domain: String,
  ) -> Result<bool, AppError> {
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/task/publish", ip_address))
      .timeout(Duration::from_secs(10))
      .json(&TaskPublishRequest {
        site_id,
        deployment_id,
        bandwidth,
        bind_domain,
        preview_domain,
      })
      .send()
      .await?;
    let data = resp.json::<RpcResponse<()>>().await.unwrap();
    if data.code == 0 {
      Ok(true)
    } else {
      error!("{}", data.msg);
      Ok(false)
    }
  }

  pub async fn task_revoke(&self, site_id: String, ip_address: &str) -> Result<bool, AppError> {
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/task/revoke", ip_address))
      .json(&TaskRevokeRequest { site_id })
      .send()
      .await?;
    let data = resp.json::<RpcResponse<()>>().await.unwrap();
    if data.code == 0 {
      Ok(true)
    } else {
      trace!("{}", data.msg);
      Ok(false)
    }
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
  Pending,
  Uploading,
  Uploaded,
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
  pub deploy_url: String,
  pub deploy_token: String,
  pub agent_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeploySiteData {
  pub domian: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssignTaskData {
  pub preview_url: String,
}

#[derive(Debug, Clone)]
pub struct MasterRpc {
  api_client: reqwest::Client,
  master_url: String,
}

impl MasterRpc {
  pub fn new(master_url: String) -> Self {
    Self {
      master_url,
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
    let data = resp.json::<RpcResponse<LoginData>>().await.unwrap();
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
    let data = resp
      .json::<RpcResponse<GetCasualTokenData>>()
      .await
      .unwrap();
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
    let data = resp.json::<RpcResponse<CreateSiteData>>().await.unwrap();
    if data.code == 0 {
      trace!("{:?}", data);
      data.data.unwrap()
    } else {
      panic!("{}", data.msg);
    }
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
    let data = resp
      .json::<RpcResponse<CreateDeploymentData>>()
      .await
      .unwrap();
    if data.code == 0 {
      trace!("{:?}", data);
      data.data.unwrap()
    } else {
      panic!("{}", data.msg);
    }
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
    resp.json::<RpcResponse<()>>().await?;
    Ok(())
  }

  pub async fn publish_site(
    &self,
    token: &str,
    site_id: String,
    deployment_id: u32,
  ) -> AssignTaskData {
    let resp = self
      .api_client
      .post(format!("{}/api/agent/task", self.master_url))
      .bearer_auth(token)
      .json(&AssignTaskRequest {
        r#type: "publish".to_string(),
        site_id,
        deployment_id,
      })
      .send()
      .await
      .unwrap();
    let data = resp.json::<RpcResponse<AssignTaskData>>().await.unwrap();
    data.data.unwrap()
  }
}

use std::net::Ipv4Addr;

use cloudflare::endpoints::dns::dns::{self, CreateDnsRecordParams};
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::client::ClientConfig;
use cloudflare::framework::client::async_api::Client;
use cloudflare::framework::{Environment, OrderDirection};

#[derive(Debug, Clone)]
pub struct CloudflareRpc {
  api_client: Client,
  zone_identifier: String,
}

impl CloudflareRpc {
  pub async fn new(zone_identifier: String, email: String, key: String) -> Self {
    let credentials = Credentials::UserAuthKey { email, key };
    let api_client = Client::new(
      credentials,
      ClientConfig::default(),
      Environment::Production,
    )
    .unwrap();
    Self {
      api_client,
      zone_identifier,
    }
  }

  pub async fn dns(&self) {
    let endpoint = dns::ListDnsRecords {
      zone_identifier: &self.zone_identifier,
      params: dns::ListDnsRecordsParams {
        direction: Some(OrderDirection::Ascending),
        ..Default::default()
      },
    };
    let response = self.api_client.request(&endpoint).await.unwrap();
    println!("{:#?}", response);
  }

  pub async fn create_a_record(&self, name: &str, ip: Ipv4Addr) {
    let endpoint = dns::CreateDnsRecord {
      zone_identifier: &self.zone_identifier,
      params: CreateDnsRecordParams {
        ttl: None,
        priority: None,
        proxied: None,
        name,
        content: dns::DnsContent::A { content: ip },
      },
    };
    if let Ok(resp) = self.api_client.request(&endpoint).await {
      println!("{:#?}", resp);
    }
  }

  pub async fn create_cname_record(&self, name: &str, content: String) {
    let endpoint = dns::CreateDnsRecord {
      zone_identifier: &self.zone_identifier,
      params: CreateDnsRecordParams {
        ttl: None,
        priority: None,
        proxied: None,
        name,
        content: dns::DnsContent::CNAME { content },
      },
    };
    let response = self.api_client.request(&endpoint).await.unwrap();
    println!("{:#?}", response);
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[tokio::test]
  pub async fn test_dns() {
    let cloudflare_api_key = std::env::var("CLOUDFLARE_API_KEY").unwrap();
    let cloudflare_email = std::env::var("CLOUDFLARE_EMAIL").unwrap();
    let cloudflare_zone_id = std::env::var("CLOUDFLARE_ZONE_ID").unwrap();
    let cf = CloudflareRpc::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key).await;
    cf.dns().await;
  }

  #[tokio::test]
  pub async fn test_create_cname_record() {
    let cloudflare_api_key = std::env::var("CLOUDFLARE_API_KEY").unwrap();
    let cloudflare_email = std::env::var("CLOUDFLARE_EMAIL").unwrap();
    let cloudflare_zone_id = std::env::var("CLOUDFLARE_ZONE_ID").unwrap();
    let cf = CloudflareRpc::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key).await;
    cf.create_cname_record("example", "root.is.me".to_string())
      .await;
  }
}
