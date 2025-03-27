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

#[derive(Debug, Deserialize)]
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

    println!("{:?}", resp.status());

    let data = resp.json::<Response<GetCasualTokenData>>().await.unwrap();
    data.data.unwrap().token
  }

  fn login() {}

  fn upload() {}

  pub async fn deploy(&self, token: String) {
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
    let data = resp.json::<Response<DeployData>>().await;
    println!("{:?}", data);
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
