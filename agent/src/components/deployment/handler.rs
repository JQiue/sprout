use actix_multipart::form::MultipartForm;
use actix_web::{
  HttpResponse, post,
  web::{Data, Json},
};

use crate::{
  app::AppState,
  components::deployment::{
    model::{InitUploadBody, UploadForm},
    service,
  },
  error::AppError,
  response::Response,
};

#[post("/upload/init")]
pub async fn init_upload(
  state: Data<AppState>,
  body: Json<InitUploadBody>,
) -> Result<HttpResponse, AppError> {
  let Json(InitUploadBody { site_id }) = body;
  match service::init_upload(&state, site_id).await {
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
