use sea_orm::prelude::StringLen;
use sea_orm::{
  ColumnTrait, DatabaseConnection, DeriveActiveEnum, EntityTrait, EnumIter, PaginatorTrait,
  QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::entities::user;
use crate::response::StatusCode;

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
  /// 普通用户
  Normal,
  /// VIP用户
  Vip,
  /// 管理员
  Admin,
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
  UserId(String),
  Email(String),
}

#[derive(Deserialize)]
pub struct SetUserPasswordBody {
  pub password: String,
}

pub async fn has_user(query_by: UserQueryBy, db: &DatabaseConnection) -> Result<bool, StatusCode> {
  let mut query = user::Entity::find();
  match query_by {
    UserQueryBy::UserId(user_id) => query = query.filter(user::Column::UserId.eq(user_id)),
    UserQueryBy::Email(email) => query = query.filter(user::Column::Email.eq(email)),
  }
  let res = query
    .one(db)
    .await
    .map_err(|_| StatusCode::DbError)?
    .ok_or(StatusCode::UserNotFound);
  Ok(res.is_ok())
}

pub async fn get_user(
  query_by: UserQueryBy,
  db: &DatabaseConnection,
) -> Result<user::Model, StatusCode> {
  let mut query = user::Entity::find();
  match query_by {
    UserQueryBy::UserId(user_id) => query = query.filter(user::Column::UserId.eq(user_id)),
    UserQueryBy::Email(email) => query = query.filter(user::Column::Email.eq(email)),
  }
  query
    .one(db)
    .await
    .map_err(|_| StatusCode::DbError)?
    .ok_or(StatusCode::UserNotFound)
}

pub async fn is_first_user(db: &DatabaseConnection) -> Result<bool, StatusCode> {
  let count = user::Entity::find()
    .count(db)
    .await
    .map_err(|_| StatusCode::DbError)?;
  Ok(count == 0)
}

pub async fn is_admin(user_id: String, db: &DatabaseConnection) -> Result<bool, StatusCode> {
  let user_type = get_user(UserQueryBy::UserId(user_id), db).await?.r#type;
  Ok(user_type == UserType::Admin)
}
