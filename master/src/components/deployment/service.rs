use serde_json::{json, Value};

use crate::app::AppState;

pub async fn user_register(
  state: &AppState,
  display_name: String,
  email: String,
  password: String,
) -> Result<Value, String> {
  Ok(json! ({
    "data": {
      "verify": true
    }
  }))
}

pub async fn user_login(
  state: &AppState,
  email: String,
  password: String,
) -> Result<Value, String> {
  Err("to do".to_string())
}

pub async fn user_logout() -> Result<Value, String> {
  Err("to do".to_string())
}
