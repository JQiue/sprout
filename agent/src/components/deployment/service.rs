use std::{fs, path::Path};

use serde_json::{Value, json};

use crate::{
  app::AppState,
  error::AppError,
  helpers::{domian::generate_domain, nginx::NginxConfig},
  rpc::MasterRpc,
};
use helpers::{self, jwt};

use super::model::UploadForm;

pub async fn init_upload(state: &AppState, site_id: String) -> Result<Value, AppError> {
  let upload_token = jwt::sign(
    site_id,
    &state.upload_token_key,
    state.upload_token_key_expire,
  )?;
  Ok(json!({
    "upload_token": upload_token,
  }))
}

pub async fn file_upload(state: &AppState, form: UploadForm) -> Result<Value, AppError> {
  jwt::verify::<String>(&form.upload_token, &state.upload_token_key)?;
  let base_dir = Path::new(&state.storage_path).join("./agent");

  if !base_dir.exists() {
    fs::create_dir_all(&base_dir)?;
  }

  for tempfile in form.dist.iter() {
    let filename = tempfile.file_name.clone().unwrap();
    let target_path = base_dir.join(filename);
    fs::copy(tempfile.file.path(), target_path)?;
  }
  MasterRpc::new(
    state.master_url.clone(),
    state.agent_token.clone(),
    state.agent_id,
  )
  .update_deployment_status()
  .await?;
  // 申请测试域名
  let domain = generate_domain();
  let nginx_root_path = base_dir.canonicalize()?;
  println!("{:?}", nginx_root_path);
  let nginx_config = NginxConfig::new(
    domain,
    nginx_root_path.to_string_lossy().to_string(),
    false,
    None,
  );
  if nginx_config.deploy(Path::new("/etc/nginx/sprout")) {
    Ok(json!({}))
  } else {
    Err(AppError::Error)
  }
}

#[cfg(test)]
mod tests {
  use super::init_upload;
  use crate::app::AppState;
  #[actix_web::test]
  async fn test_init_upload() {
    let state = AppState {
      agent_id: 1,
      agent_token: "kjfklfa".to_string(),
      storage_path: "./".to_string(),
      master_url: "127..0.1.0".to_string(),
      upload_token_key: "efkalwfewalkf".to_string(),
      upload_token_key_expire: 1000,
    };
    let res = init_upload(&state, "alfjalfafj".to_string()).await;
    if let Ok(value) = res {
      // 断言返回的 JSON 包含上传令牌
      assert!(value.get("upload_token").is_some());
      println!("Upload token: {}", value["upload_token"]);
    } else {
      // 如果有错误发生，则断言失败并提供更多信息
      panic!("Failed to initialize upload: {:?}", res.err().unwrap());
    }
  }
  // #[actix_web::test]
  // async fn test_file_upload_ok() {
  //   let state = AppState {
  //     agent_id: 1,
  //     storage_path: "./tests/data".to_string(),
  //     master_url: "http://127.0.0.1".to_string(),
  //     upload_token_key: "efkalwfewalkf".to_string(),
  //     upload_token_key_expire: 1000,
  //   };
  // }
}
