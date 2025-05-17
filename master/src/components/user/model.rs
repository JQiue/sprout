use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetUserPasswordBody {
  pub password: String,
}
