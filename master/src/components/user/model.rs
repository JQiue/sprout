use sea_orm::prelude::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct SetUserPasswordBody {
  pub password: String,
}
