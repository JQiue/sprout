use std::time::Duration;

use entity::deployment;
use sea_orm::prelude::StringLen;
use sea_orm::{DatabaseConnection, DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

use crate::components::agent::model::get_agent;
use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
#[serde(rename_all = "lowercase")]
pub enum SiteStatus {
  Active,
  Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
  Imported,
  Template,
  Manual,
}

#[derive(Deserialize)]
pub struct CreateSiteBody {
  pub site_name: String,
  // pub site_type: SiteType,
  // pub repo_url: Option<String>,
  // pub template_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DeploySource {
  Manual,     // 手动上传部署
  Template,   // 从模板创建
  Repository, // 从代码仓库部署
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadBodyData {
  pub upload_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadBody {
  pub code: i32,
  pub msg: String,
  pub data: InitUploadBodyData,
}

pub async fn init_upload_session(
  agent_ip: String,
  deployment: deployment::Model,
) -> Result<InitUploadBody, AppError> {
  let resp = reqwest::Client::new()
    .post(format!("http://{}/api/upload/init", agent_ip))
    .timeout(Duration::from_secs(3))
    .json(&json!({
      "site_id": deployment.site_id,
      "deploy_id": deployment.id
    }))
    .send()
    .await?;
  let data = resp.json::<InitUploadBody>().await?;
  error!("Response body: {:?}", data);
  Ok(data)
}
