use serde_json::{json, Value};

use crate::app::AppState;

pub async fn register_server(state: &AppState) -> Result<Value, String> {
  Ok(json! ({
    "data": {
      "verify": true
    }
  }))
}
