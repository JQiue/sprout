//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::EnumIter;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
pub enum UserStatus {
  Active,  // 正常
  Deleted, // 已删除
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
pub enum UserType {
  /// 临时用户
  Casual,
  /// VIP 用户
  Normal,
  /// 管理员
  Administrator,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: u32,
  #[sea_orm(unique)]
  pub user_id: String,
  pub nickname: String,
  pub password: String,
  pub email: String,
  pub r#type: UserType,
  pub status: UserStatus,
  pub is_email_verified: i8,
  pub is_phone_verified: i8,
  pub created_at: DateTimeUtc,
  pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
