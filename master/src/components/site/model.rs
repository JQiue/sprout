use sea_orm::prelude::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

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
