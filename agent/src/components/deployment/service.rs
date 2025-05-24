use std::{fs, path::Path};

use common::agent::InitUploadResponse;
use serde_json::Value;
use tracing::debug;

use crate::{
  app::AppState,
  error::AppError,
  helper::{NginxConfig, check_dns_record, extract_tar},
  types::ServiceResult,
};
use helpers::{self, jwt};

use super::model::UploadForm;

pub async fn get_upload_token(
  state: &AppState,
  site_id: String,
) -> ServiceResult<InitUploadResponse> {
  let upload_token = jwt::sign(
    site_id,
    &state.upload_token_key,
    state.upload_token_key_expire,
  )?;
  Ok(InitUploadResponse { upload_token })
}

pub async fn file_upload(state: &AppState, form: UploadForm) -> ServiceResult<Value> {
  jwt::verify::<String>(&form.upload_token, &state.upload_token_key)?;
  let base_dir = Path::new(&state.storage_path);

  if !base_dir.exists() {
    fs::create_dir_all(base_dir)?;
  }

  for tempfile in form.dist.iter() {
    let filename = tempfile
      .file_name
      .clone()
      .ok_or(AppError::TempfileNotFound)?;
    let target_path = base_dir.join(filename);
    fs::copy(tempfile.file.path(), target_path)?;
  }

  Ok(Value::Null)
}

pub async fn publish_site(
  state: &AppState,
  site_id: String,
  bandwidth: String,
  bind_domain: Option<String>,
  preview_domain: String,
) -> ServiceResult<Value> {
  let base_dir = Path::new(&state.storage_path);

  if !base_dir.exists() {
    fs::create_dir_all(base_dir)?;
  }

  let nginx_root_path = format!("{}/{}", base_dir.canonicalize()?.to_string_lossy(), site_id);
  // 解压 tar
  if !extract_tar(
    nginx_root_path.clone() + ".tar",
    base_dir.canonicalize()?.to_string_lossy().to_string(),
  ) {
    return Err(AppError::ExtractTar);
  };

  debug!("nginx_root_path: {:?}", nginx_root_path);
  let mut nginx_config = NginxConfig::new(&state.nginx_config_path, false);
  let server_name = if let Some(bind_domain) = bind_domain {
    if check_dns_record(&bind_domain, state.public_ip)? {
      nginx_config.ssl_enabled = true;
    }
    [bind_domain, preview_domain].join(" ")
  } else {
    preview_domain
  };

  if nginx_config.deploy(&server_name, &nginx_root_path, &bandwidth, &site_id) {
    Ok(Value::Null)
  } else {
    Err(AppError::NginxDeploy)
  }
}

pub async fn revoke_site(state: &AppState, site_id: String) -> ServiceResult<Value> {
  let base_dir = Path::new(&state.storage_path);
  fs::remove_dir_all(base_dir.join(&site_id))?;
  let nc = NginxConfig::new(&state.nginx_config_path, false);
  nc.remove_config(&site_id)?;
  Ok(Value::Null)
}

#[cfg(test)]
mod tests {
  use super::get_upload_token;
  use crate::app::AppState;
  #[actix_web::test]
  async fn test_get_upload_token() {
    let state = AppState {
      nginx_config_path: "/etc/nginx/sprout".to_string(),
      storage_path: "./".to_string(),
      upload_token_key: "efkalwfewalkf".to_string(),
      upload_token_key_expire: 1000,
      public_ip: "192.168.5.12".parse().unwrap(),
    };

    match get_upload_token(&state, "alfjalfafj".to_string()).await {
      Ok(res) => {
        println!("Upload token: {}", res.upload_token);
      }
      Err(err) => panic!("Failed to initialize upload: {:?}", err),
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
