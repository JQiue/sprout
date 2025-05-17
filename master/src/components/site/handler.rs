use actix_web::{
  HttpRequest, HttpResponse, get, post,
  web::{Data, Json},
};
use helpers::jwt;

use crate::{
  app::AppState,
  components::site::{model::*, service},
  error::AppError,
  helper::extract_token,
  traits::IntoHttpResponse,
};

#[post("/site")]
pub async fn create_site(
  req: HttpRequest,
  state: Data<AppState>,
  body: Json<CreateSiteBody>,
) -> Result<HttpResponse, AppError> {
  let token = extract_token(&req)?;
  let user_id = jwt::verify::<String>(&token, &state.login_token_key)?
    .claims
    .data;
  service::create_site(&state, user_id, body.0.site_name)
    .await
    .into_http_response()
}

#[get("/sites")]
pub async fn get_sites(req: HttpRequest, state: Data<AppState>) -> Result<HttpResponse, AppError> {
  let token = extract_token(&req)?;
  let user_id = jwt::verify::<String>(&token, &state.login_token_key)?
    .claims
    .data;
  service::get_sites(&state, user_id)
    .await
    .into_http_response()
}
