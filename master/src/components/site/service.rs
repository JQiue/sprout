use crate::{app::AppState, types::ServiceResult};
use common::master::GetSitesResponse;
use entity::site;
use helpers::{
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use sea_orm::Set;
use serde_json::{Value, json};

/// Creates a new site for a user.
///
/// # Arguments
///
/// * `state` - A reference to the application state.
/// * `user_id` - The unique identifier of the user creating the site.
/// * `site_name` - The name of the site.
/// * `site_type` - The type of site being created.
/// * `repo_url` - An optional URL to the repository associated with the site.
///
/// # Returns
///
/// A `Result` containing a `Value` with the site creation status on success,
/// or a `String` error message on failure.
pub async fn create_site(
  state: &AppState,
  user_id: String,
  site_name: String,
) -> ServiceResult<Value> {
  let site = state
    .repo
    .site()
    .create_site(site::ActiveModel {
      site_id: Set(nanoid(&Alphabet::LOWER, 20)),
      name: Set(site_name),
      user_id: Set(user_id),
      bandwidth: Set(site::Bandwidth::One),
      created_at: Set(utc_now()),
      ..Default::default()
    })
    .await?;
  Ok(json!({
    "site_id": site.site_id,
    "name": site.name,
  }))
}

pub async fn get_sites(state: &AppState, user_id: String) -> ServiceResult<GetSitesResponse> {
  let sites = state.repo.site().get_sites_by_user_id(user_id).await?;
  Ok(GetSitesResponse { sites })
}
