use actix_web::{
  post,
  web::{Data, Json, ReqData},
  HttpResponse,
};

use crate::{
  app::AppState,
  components::site::{model::*, service},
  middleware::auth::JwtPayload,
  response::Response,
};

#[post("/site")]
pub async fn create_site(
  state: Data<AppState>,
  body: Json<CreateSiteBody>,
  req_data: ReqData<JwtPayload>,
) -> HttpResponse {
  let Json(CreateSiteBody {
    site_name,
    site_type,
    repo_url,
    template_id,
  }) = body;
  match service::create_site(
    &state,
    req_data.user_id.clone(),
    site_name,
    site_type,
    repo_url,
    template_id,
  )
  .await
  {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
