use std::{fs, path::Path};

use serde_json::{json, Value};

use crate::{
  app::AppState,
  helpers::{
    audit_file::{audit_directory, load_keywords_from_files},
    domian::generate_domain,
    nginx::{create_nginx_server_80, NginxConfig},
  },
  response::StatusCode,
};
use helpers;

use super::model::{is_safe_path, UploadForm};

pub async fn init_upload(_state: &AppState) -> Result<Value, StatusCode> {
  let token = helpers::jwt::sign("upload_file".to_string(), "agent".to_owned(), 1000)
    .map_err(|_| StatusCode::ServerError)?;
  Ok(json!({
    "upload_token": token,
  }))
}

pub async fn file_upload(state: &AppState, form: UploadForm) -> Result<Value, StatusCode> {
  helpers::jwt::verify::<String>(form.upload_token.to_string(), "agent_key".to_owned()).map_err(
    |error| {
      println!("{:?}", error);
      StatusCode::UploadTokenError
    },
  )?;

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
  // 审核通过后开始部署到 Nginx

  // 申请测试域名
  let domain = generate_domain();

  // create_nginx_server_80(domain);
  let nginx_config = NginxConfig::new(domain, base_dir.to_string_lossy().to_string(), false, None);
  nginx_config
    .deploy(Path::new("/etc/nginx/sprout"))
    .map_err(|_| StatusCode::DeployError)?;
  Ok(json!({}))
}
