use actix_web::{
  HttpResponse, get, post,
  web::{Data, Json, ReqData},
};
use common::master::UserLoginReqeust;

use crate::{
  app::AppState,
  components::user::{model::*, service},
  error::AppError,
  middlewares::JwtPayload,
  response::Response,
};

#[get("/user/casual")]
pub async fn generate_casual_user(state: Data<AppState>) -> Result<HttpResponse, AppError> {
  match service::generate_casual_user(&state).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/user")]
pub async fn user_register(
  state: Data<AppState>,
  body: Json<UserRegisterBody>,
) -> Result<HttpResponse, AppError> {
  let Json(UserRegisterBody {
    nickname,
    password,
    email,
  }) = body;

  match service::user_register(&state, nickname, email, password).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/user/token")]
pub async fn user_login(
  state: Data<AppState>,
  body: Json<UserLoginReqeust>,
) -> Result<HttpResponse, AppError> {
  match service::user_login(&state, body.0.email, body.0.password).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[get("/user/info")]
pub async fn get_user_info(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
) -> Result<HttpResponse, AppError> {
  match service::get_user_info(&state, req_data.user_id.clone()).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/user/password")]
pub async fn set_user_password(
  state: Data<AppState>,
  body: Json<SetUserPasswordBody>,
  req_data: ReqData<JwtPayload>,
) -> Result<HttpResponse, AppError> {
  let Json(SetUserPasswordBody { password }) = body;
  match service::set_user_password(&state, req_data.user_id.clone(), password).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/token/refresh")]
pub async fn refresh_user_token(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
) -> Result<HttpResponse, AppError> {
  match service::refresh_user_token(&state, req_data.user_id.clone()).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
