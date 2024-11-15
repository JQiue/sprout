use sea_orm::prelude::StringLen;
use sea_orm::{
  ColumnTrait, DatabaseConnection, DeriveActiveEnum, EntityTrait, EnumIter, QueryFilter,
};
use serde::Deserialize;

use crate::entitys::{prelude::User, user};

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

#[derive(Deserialize, Clone)]
pub struct UserRegisterBody {
  pub nickname: String,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct UserLoginBody {
  pub email: String,
  pub password: String,
}

pub enum UserQueryBy {
  UserId(u32),
  Email(String),
}

pub async fn has_user(query_by: UserQueryBy, db: &DatabaseConnection) -> bool {
  let mut query = User::find();
  match query_by {
    UserQueryBy::UserId(user_id) => query = query.filter(user::Column::UserId.eq(user_id)),
    UserQueryBy::Email(email) => query = query.filter(user::Column::Email.eq(email)),
  }
  let res = query.one(db).await.unwrap();
  res.is_some()
}

pub async fn get_user(query_by: UserQueryBy, db: &DatabaseConnection) -> user::Model {
  let mut query = User::find();
  match query_by {
    UserQueryBy::UserId(user_id) => query = query.filter(user::Column::UserId.eq(user_id)),
    UserQueryBy::Email(email) => query = query.filter(user::Column::Email.eq(email)),
  }

  let res = query.one(db).await.unwrap().unwrap();
  res
}
