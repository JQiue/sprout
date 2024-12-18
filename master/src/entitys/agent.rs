//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::components::agent::model::AgentStatus;

#[derive(Serialize, Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "agent")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub hostname: String,
  pub ip_address: String,
  pub storage_path: String,
  pub available_space: u32,
  pub status: AgentStatus,
  pub tags: Option<String>,
  pub token: String,
  pub created_at: DateTimeUtc,
  pub updated_at: Option<DateTimeUtc>,
  pub last_heartbeat: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
