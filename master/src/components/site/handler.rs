use actix_web::{
  HttpRequest, HttpResponse, post,
  web::{Data, Json},
};
use helpers::jwt;

use crate::{
  app::AppState,
  components::site::{model::*, service},
  error::AppError,
  helper::extract_token,
  response::Response,
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
  match service::create_site(&state, user_id, body.0.site_name).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
