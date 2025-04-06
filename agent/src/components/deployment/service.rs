use std::{fs, path::Path};

use rpc::Master::DeploymentStatus;
use serde_json::{Value, json};
use tracing::trace;

use crate::{
  app::AppState,
  error::AppError,
  helper::{NginxConfig, extract_tar, generate_domian},
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
  jwt::verify::<String>(&form.upload_token, &state.upload_token_key)?
    .claims
    .data;
  let base_dir = Path::new(&state.storage_path);

  if !base_dir.exists() {
    fs::create_dir_all(&base_dir)?;
  }

  for tempfile in form.dist.iter() {
    let filename = tempfile.file_name.clone().unwrap();
    let target_path = base_dir.join(filename);
    fs::copy(tempfile.file.path(), target_path)?;
  }

  state
    .master_rpc
    .update_deployment_status(
      state.agent_token.clone(),
      *form.deployment_id,
      DeploymentStatus::Uploaded,
    )
    .await?;

  Ok(Value::Null)
}

pub async fn publish_site(
  state: &AppState,
  site_id: String,
  deployment_id: u32,
) -> Result<Value, AppError> {
  let base_dir = Path::new(&state.storage_path);

  if !base_dir.exists() {
    fs::create_dir_all(&base_dir)?;
  }

  // 申请预览域名
  let domian = generate_domian(&format!("preview_{site_id}"));
  let nginx_root_path = format!(
    "{}/{}",
    base_dir.canonicalize()?.to_string_lossy().to_string(),
    site_id
  );
  // 解压 tar
  extract_tar(
    nginx_root_path.clone() + ".tar",
    base_dir.canonicalize()?.to_string_lossy().to_string(),
  );
  trace!("{:?}", nginx_root_path);
  let nginx_config = NginxConfig::new(domian.clone(), nginx_root_path, false, None);

  if nginx_config.deploy(Path::new("/etc/nginx/sprout")) {
    state
      .master_rpc
      .update_deployment_status(
        state.agent_token.clone(),
        deployment_id,
        DeploymentStatus::Published,
      )
      .await?;
    Ok(json!({ "domian": domian }))
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
      master_rpc: rpc::Master::Rpc::new("http://127.0.0.1:3000".to_string()),
      agent_token: "kjfklfa".to_string(),
      storage_path: "./".to_string(),
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
