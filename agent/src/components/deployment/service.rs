use std::{fs, path::Path};

use serde_json::{json, Value};

use crate::{
  app::AppState,
  helpers::{
    audit_file::{audit_directory, load_keywords_from_files},
    domian::generate_domain,
    nginx::NginxConfig,
  },
  response::StatusCode,
};
use helpers::{self, jwt};

use super::model::{is_safe_path, UploadForm};

pub async fn init_upload(state: &AppState, site_id: String) -> Result<Value, StatusCode> {
  let token = jwt::sign(
    site_id,
    state.upload_token_key.clone(),
    state.upload_token_key_expire,
  )
  .map_err(|_| StatusCode::ServerError)?;
  Ok(json!({
    "upload_token": token,
  }))
}

pub async fn file_upload(state: &AppState, form: UploadForm) -> Result<Value, StatusCode> {
  jwt::verify::<String>(
    form.upload_token.to_string(),
    state.upload_token_key.to_owned(),
  )
  .map_err(|error| {
    tracing::error!("{}", error);
    StatusCode::UploadTokenError
  })?;

  let base_dir = Path::new(&state.storage_path).join("./agent/upload");
  if !base_dir.exists() {
    fs::create_dir_all(&base_dir).map_err(|_| StatusCode::FileSystemError)?;
  }
  for (metadata, temp_file) in form.json.iter().zip(form.files.iter()) {
    if !is_safe_path(&metadata.path) {
      return Err(StatusCode::UploadError);
    }
    let target_path = base_dir.join(metadata.path.trim_start_matches('/'));
    if let Some(parent) = target_path.parent() {
      if !parent.exists() {
        fs::create_dir_all(parent).map_err(|_| StatusCode::UploadError)?;
      }
    }
    fs::copy(temp_file.file.path(), target_path).map_err(|_| StatusCode::UploadError)?;
  }
  let resp = reqwest::Client::new()
    .post(format!("{}/api/deployment/status", state.master_url))
    .json(&json!({
      "agent_id": state.agent_id,
      "agent_token": form.upload_token.to_string(),
      "deployment_id": 1,
      "status": "reviewing"
    }))
    .send()
    .await
    .map_err(|_| StatusCode::SendMasterRequestError)?;
  if !resp.status().is_success() {
    return Err(StatusCode::SendMasterRequestError);
  }
  let keyword_files = vec![
    "./agent/涉枪涉爆违法信息关键词.txt",
    "./agent/色情类.txt",
    "./agent/政治类.txt",
  ];
  let keywords =
    load_keywords_from_files(&keyword_files).map_err(|_| StatusCode::LoadKeywordsError)?;
  let res = audit_directory(&base_dir, &keywords).map_err(|_| StatusCode::FileSystemError)?;
  if res.len() != 0 {
    return Err(StatusCode::DeployError);
  }
  // 申请测试域名
  let domain = generate_domain();
  let nginx_root_path = base_dir
    .canonicalize()
    .map_err(|_| StatusCode::FileSystemError)?;
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
    Err(StatusCode::DeployError)
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
      storage_path: "./".to_string(),
      master_url: "127..0.1.0".to_string(),
      upload_token_key: "efkalwfewalkf".to_string(),
      upload_token_key_expire: 1000,
    };
    let res = init_upload(&state, "alfjalfafj".to_string()).await;
    // 使用 match 或者直接解包 Result 来处理可能的错误
    if let Ok(value) = res {
      // 断言返回的 JSON 包含上传令牌
      assert!(value.get("upload_token").is_some());
      println!("Upload token: {}", value["upload_token"]);
    } else {
      // 如果有错误发生，则断言失败并提供更多信息
      panic!("Failed to initialize upload: {:?}", res.err().unwrap());
    }
  }
  #[actix_web::test]
  async fn test_file_upload_ok() {
    let state = AppState {
      agent_id: 1,
      storage_path: "./tests/data".to_string(),
      master_url: "http://127.0.0.1".to_string(),
      upload_token_key: "efkalwfewalkf".to_string(),
      upload_token_key_expire: 1000,
    };
  }
}
