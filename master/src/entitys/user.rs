//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
pub enum UserStatus {
  Active,  // 正常
  Deleted, // 已删除
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
pub enum UserType {
  Normal, // 普通用户
  Vip,    // VIP用户
  Admin,  // 管理员
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  #[sea_orm(unique)]
  pub user_id: String,
  pub nickname: String,
  pub password: String,
  pub email: String,
  pub avatar: Option<String>,
  pub r#type: UserType,
  pub status: UserStatus,
  pub is_email_verified: i8,
  pub is_phone_verified: i8,
  pub failed_login_attempts: i8,
  pub last_login_ip: Option<String>,
  pub last_login_at: Option<DateTimeUtc>,
  pub created_at: DateTimeUtc,
  pub updated_at: Option<DateTimeUtc>,
  pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}