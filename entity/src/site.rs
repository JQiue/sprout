//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
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

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "site")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: u32,
  pub site_id: String,
  pub user_id: String,
  pub server_id: Option<u32>,
  pub name: String,
  pub domain: Option<String>,
  pub status: SiteStatus,
  pub created_at: DateTimeUtc,
  pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
