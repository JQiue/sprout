use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRegisterBody {
  pub display_name: String,
  pub email: String,
  pub password: String,
}

#[derive(Deserialize)]
pub struct UserLoginBody {
  pub email: String,
  pub password: String,
}
