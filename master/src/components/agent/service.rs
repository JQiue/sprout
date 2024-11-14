use serde_json::{json, Value};

use crate::{app::AppState, response::StatusCode};

pub async fn register_agent(state: &AppState) -> Result<Value, StatusCode> {
  Ok(json! ({
    "data": {
      "verify": true
    }
  }))
}
