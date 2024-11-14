use actix_web::{
  post,
  web::{Data, Json, ReqData},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::site::{model::*, service},
  middleware::auth::JwtPayload,
};

#[post("/site")]
pub async fn create_site(
  state: Data<AppState>,
  body: Json<CreateSiteBody>,
  req_data: ReqData<JwtPayload>,
) -> HttpResponse {
  let Json(CreateSiteBody {
    site_type,
    repo_url,
  }) = body;

  match service::create_site(&state, req_data.user_id.clone(), site_type, repo_url).await {
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
