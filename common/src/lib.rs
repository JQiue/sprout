use serde::{Deserialize, Serialize};

pub mod agent;
pub mod master;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  pub code: i32,
  pub msg: String,
  pub data: T,
}
