use sea_orm::prelude::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
pub enum SiteStatus {
  Active,
  Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
pub enum SiteType {
  Imported,
  Template,
  Manual,
}

#[derive(Deserialize)]
pub struct UserRegisterBody {
  pub display_name: String,
  pub email: String,
  pub password: String,
}
