use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UserRegisterBody {
  pub nickname: String,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct SetUserPasswordBody {
  pub password: String,
}
