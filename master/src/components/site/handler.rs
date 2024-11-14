use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::site::{model::*, service},
};

#[post("/site")]
pub async fn create_site(state: Data<AppState>, body: Json<CreateSiteBody>) -> HttpResponse {
  let Json(CreateSiteBody {
    user_id,
    site_type,
    repo_url,
  }) = body;

  match service::create_site(&state, user_id, site_type, repo_url).await {
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
