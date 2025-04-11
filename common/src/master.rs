use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserLoginReqeust {
  pub email: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssignTaskRequest {
  pub r#type: String,
  pub site_id: String,
  pub deployment_id: u32,
}
