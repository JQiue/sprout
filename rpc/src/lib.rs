pub mod error;

use std::fmt::Debug;
use std::{path::PathBuf, time::Duration};

use common::{
  agent::{
    HeartbeatResponse, InitUploadRequest, InitUploadResponse, TaskPublishRequest, TaskRevokeRequest,
  },
  master::{
    AssignTaskRequest, CreateDeploymentRequest, CreateDeploymentResponse, GetSitesResponse,
    UserRegisterRequest,
  },
};

use reqwest::header::CONTENT_TYPE;
use reqwest::{
  Response,
  header::{ACCEPT, HeaderMap},
  multipart::{Form, Part},
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse<T> {
  pub code: i32,
  pub msg: String,
  // #[serde(skip_serializing_if = "Option::is_none")]
  pub data: T,
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
  pub fn new() -> Result<Self, Error> {
    Ok(Self {
      api_client: reqwest::Client::builder()
        .default_headers({
          let mut headers = HeaderMap::new();
          headers.insert(ACCEPT, "application/json".parse().unwrap());
          headers
        })
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|_| Error::BuildRequest)?,
    })
  }

  pub async fn fetch<T: Serialize, B: DeserializeOwned>(
    &self,
    agent_ip: &str,
    method: &str,
    path: &str,
    body: Option<T>,
  ) -> Result<B, Error> {
    let url = format!("http://{}:5001/api/{}", agent_ip, path);
    let client = if method == "post" {
      self.api_client.post(url).json(&body)
    } else {
      self.api_client.get(url)
    };
    let resp = client.send().await?;
    let content_type = resp
      .headers()
      .get(CONTENT_TYPE)
      .ok_or(Error::InvalidContentType)?;
    if content_type != "application/json" {
      return Err(Error::InvalidContentType);
    }

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<B>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn get_agent_heartbeat(&self, agent_ip: &str) -> Result<HeartbeatResponse, Error> {
    let body = self
      .fetch::<_, HeartbeatResponse>(agent_ip, "get", "heartbeat", Some(()))
      .await?;
    Ok(body)
  }

  pub async fn init_upload_session(
    &self,
    agent_ip: &str,
    site_id: String,
  ) -> Result<InitUploadResponse, Error> {
    let body = self
      .fetch::<_, InitUploadResponse>(
        agent_ip,
        "post",
        "upload/init",
        Some(InitUploadRequest { site_id }),
      )
      .await?;
    Ok(body)
  }

  pub async fn upload_file(
    &self,
    agent_ip: &str,
    upload_token: String,
    deployment_id: u32,
    path: PathBuf,
  ) -> Result<(), Error> {
    let path_buf = path.clone();
    let file_name = path
      .file_name()
      .and_then(|n| n.to_str())
      .map(|s| s.to_string())
      .unwrap_or_else(|| "unknown".to_string());
    let part = Part::file(path_buf)
      .await?
      .file_name(file_name)
      .mime_str("application/octet-stream")?;
    let mut form = Form::new().part("dist", part);
    form = form.part("upload_token", Part::text(upload_token));
    form = form.part("deployment_id", Part::text(deployment_id.to_string()));
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/upload/file", agent_ip))
      .multipart(form)
      .send()
      .await?;
    if resp.status().is_success() {
      resp.json::<RpcResponse<()>>().await?;
      Ok(())
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
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
  ) -> Result<bool, Error> {
    self
      .fetch::<_, Value>(
        &ip_address,
        "post",
        "task/publish",
        Some(TaskPublishRequest {
          site_id,
          deployment_id,
          bandwidth,
          bind_domain,
          preview_domain,
        }),
      )
      .await?;
    Ok(true)

    // if resp.status().is_success() {
    //   resp.json::<RpcResponse<()>>().await?;
    //   Ok(true)
    // } else {
    //   let status_code = resp.status().as_u16();
    //   let data = resp.json::<RpcResponse<Value>>().await?;
    //   Err(Error::Api(status_code, data.code, data.msg))
    // }
  }

  pub async fn task_revoke(&self, site_id: String, ip_address: &str) -> Result<bool, Error> {
    let resp = self
      .api_client
      .post(format!("http://{}:5001/api/task/revoke", ip_address))
      .json(&TaskRevokeRequest { site_id })
      .send()
      .await?;
    if resp.status().is_success() {
      resp.json::<RpcResponse<()>>().await?;
      Ok(true)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
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
  pub token: String,
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
  pub fn new(master_url: String) -> Result<Self, Error> {
    Ok(Self {
      master_url,
      api_client: reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()?,
    })
  }

  pub async fn fetch<T: Serialize, B: DeserializeOwned>(
    &self,
    method: &str,
    path: &str,
    body: Option<T>,
  ) -> Result<B, Error> {
    let url = format!("{}/api{}", self.master_url, path);
    let client = if method == "post" {
      self.api_client.post(url).json(&body)
    } else {
      self.api_client.get(url)
    };
    let resp = client.send().await?;
    let content_type = resp
      .headers()
      .get(CONTENT_TYPE)
      .ok_or(Error::InvalidContentType)?;
    if content_type != "application/json" {
      return Err(Error::InvalidContentType);
    }

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<B>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn signup(
    &self,
    nickname: String,
    email: String,
    password: String,
  ) -> Result<bool, Error> {
    let resp = self
      .api_client
      .post(format!("{}/api/user", self.master_url))
      .json(&UserRegisterRequest {
        nickname,
        email,
        password,
      })
      .send()
      .await?;

    if resp.status().is_success() {
      resp.json::<RpcResponse<()>>().await?;
      Ok(true)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn login(&self, email: String, password: String) -> Result<LoginData, Error> {
    let body = self
      .fetch::<_, LoginData>(
        "post",
        "/user/token",
        Some(json!({
          "email": email,
          "password": password,
        })),
      )
      .await?;
    Ok(body)
  }

  pub async fn get_casual_token(&self) -> Result<GetCasualTokenData, Error> {
    let resp = self
      .api_client
      .get(format!("{}/api/user/casual", self.master_url))
      .send()
      .await?;

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<GetCasualTokenData>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn create_site(&self, token: &str) -> Result<CreateSiteData, Error> {
    let resp = self
      .api_client
      .post(format!("{}/api/site", self.master_url))
      .bearer_auth(token)
      .json(&json!({
        "site_name": "casual_site"
      }))
      .send()
      .await?;

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<CreateSiteData>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn get_sites(&self, token: &str) -> Result<GetSitesResponse, Error> {
    let resp = self
      .api_client
      .get(format!("{}/api/sites", self.master_url))
      .bearer_auth(token)
      .send()
      .await?;

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<GetSitesResponse>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn create_deployment(
    &self,
    site_id: String,
    token: &str,
  ) -> Result<CreateDeploymentResponse, Error> {
    let resp = self
      .api_client
      .post(format!("{}/api/deployment", self.master_url))
      .bearer_auth(token)
      .json(&CreateDeploymentRequest { site_id })
      .send()
      .await?;

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<CreateDeploymentResponse>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn update_deployment_status(
    &self,
    agent_token: String,
    deployment_id: u32,
    status: DeploymentStatus,
  ) -> Result<(), Error> {
    let resp = self
      .api_client
      .post(format!("{}/api/deployment/status", self.master_url))
      .json(&json!({
        "agent_token": agent_token,
        "deployment_id": deployment_id,
        "status": status
      }))
      .send()
      .await?;

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<()>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
  }

  pub async fn publish_site(
    &self,
    token: &str,
    site_id: String,
    deployment_id: u32,
    bind_domain: Option<String>,
  ) -> Result<AssignTaskData, Error> {
    let resp = self
      .api_client
      .post(format!("{}/api/agent/task", self.master_url))
      .bearer_auth(token)
      .json(&AssignTaskRequest {
        r#type: "publish".to_string(),
        site_id,
        deployment_id,
        bind_domain,
      })
      .send()
      .await?;

    if resp.status().is_success() {
      let data = resp.json::<RpcResponse<AssignTaskData>>().await?;
      Ok(data.data)
    } else {
      let status_code = resp.status().as_u16();
      let data = resp.json::<RpcResponse<Value>>().await?;
      Err(Error::Api(status_code, data.code, data.msg))
    }
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
  pub async fn new(
    zone_identifier: String,
    email: String,
    key: String,
  ) -> Result<CloudflareRpc, Error> {
    let credentials = Credentials::UserAuthKey { email, key };
    let api_client = Client::new(
      credentials,
      ClientConfig::default(),
      Environment::Production,
    )?;
    Ok(Self {
      api_client,
      zone_identifier,
    })
  }

  pub async fn dns(&self) -> Result<Vec<dns::DnsRecord>, Error> {
    let endpoint = dns::ListDnsRecords {
      zone_identifier: &self.zone_identifier,
      params: dns::ListDnsRecordsParams {
        direction: Some(OrderDirection::Ascending),
        ..Default::default()
      },
    };
    let response = self.api_client.request(&endpoint).await?;
    Ok(response.result)
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

  pub async fn create_cname_record(
    &self,
    name: &str,
    content: String,
  ) -> Result<dns::DnsRecord, Error> {
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
    let response = self.api_client.request(&endpoint).await?;
    Ok(response.result)
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
    let cf = CloudflareRpc::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key)
      .await
      .unwrap();
    cf.dns().await;
  }

  #[tokio::test]
  pub async fn test_create_cname_record() {
    let cloudflare_api_key = std::env::var("CLOUDFLARE_API_KEY").unwrap();
    let cloudflare_email = std::env::var("CLOUDFLARE_EMAIL").unwrap();
    let cloudflare_zone_id = std::env::var("CLOUDFLARE_ZONE_ID").unwrap();
    let cf = CloudflareRpc::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key)
      .await
      .unwrap();
    cf.create_cname_record("example", "root.is.me".to_string())
      .await;
  }
}
