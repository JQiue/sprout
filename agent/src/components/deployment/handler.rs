use actix_multipart::form::MultipartForm;
use actix_web::{
  HttpResponse, post,
  web::{Data, Json},
};
use common::agent::{InitUploadRequest, TaskPublishRequest, TaskRevokeRequest};

use crate::{
  app::AppState,
  components::deployment::{model::UploadForm, service},
  error::AppError,
  response::Response,
};

#[post("/upload/init")]
pub async fn init_upload(
  state: Data<AppState>,
  body: Json<InitUploadRequest>,
) -> Result<HttpResponse, AppError> {
  match service::init_upload(&state, body.0.site_id).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/upload/file")]
pub async fn file_upload(
  state: Data<AppState>,
  MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, AppError> {
  match service::file_upload(&state, form).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/task/publish")]
pub async fn publish_site(
  state: Data<AppState>,
  body: Json<TaskPublishRequest>,
) -> Result<HttpResponse, AppError> {
  match service::publish_site(
    &state,
    body.0.site_id,
    body.0.bandwidth,
    body.0.bind_domain,
    body.0.preview_domain,
  )
  .await
  {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/task/revoke")]
pub async fn revoke_site(
  state: Data<AppState>,
  body: Json<TaskRevokeRequest>,
) -> Result<HttpResponse, AppError> {
  match service::revoke_site(&state, body.0.site_id).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
