use actix_web::{
  get, post,
  web::{Data, Json, ReqData},
  HttpResponse,
};

use crate::{
  app::AppState,
  components::user::{model::*, service},
  middleware::auth::JwtPayload,
  response::Response,
};

#[post("/user")]
pub async fn user_register(state: Data<AppState>, body: Json<UserRegisterBody>) -> HttpResponse {
  let Json(UserRegisterBody {
    nickname,
    password,
    email,
  }) = body;

  match service::user_register(&state, nickname, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/user/token")]
pub async fn user_login(state: Data<AppState>, body: Json<UserLoginBody>) -> HttpResponse {
  let Json(UserLoginBody { email, password }) = body;
  match service::user_login(&state, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[get("/user/info")]
pub async fn get_user_info(state: Data<AppState>, req_data: ReqData<JwtPayload>) -> HttpResponse {
  match service::get_user_info(&state, req_data.user_id.clone()).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/user/password")]
pub async fn set_user_password(
  state: Data<AppState>,
  body: Json<SetUserPasswordBody>,
  req_data: ReqData<JwtPayload>,
) -> HttpResponse {
  let Json(SetUserPasswordBody { password }) = body;
  match service::set_user_password(&state, req_data.user_id.clone(), password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/token/refresh")]
pub async fn refresh_user_token(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
) -> HttpResponse {
  match service::refresh_user_token(&state, req_data.user_id.clone()).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
