use actix_web::{
  delete,
  http::header::HeaderValue,
  post,
  web::{Data, Json},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::site::{model::*, service},
};

#[post("/user")]
pub async fn user_register(state: Data<AppState>, body: Json<UserRegisterBody>) -> HttpResponse {
  let Json(UserRegisterBody {
    email,
    password,
    display_name,
  }) = body;

  match service::user_register(&state).await {
    Ok(data) => HttpResponse::Ok().json(json!({
     "data": data,
     "errmsg": "",
     "errno": 0,
    })),
    Err(err) => HttpResponse::Ok().json(json!({
     "errmsg": err,
     "errno": 1000,
    })),
  }
}
