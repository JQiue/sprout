use super::model::SiteType;
use crate::{app::AppState, response::StatusCode};
use serde_json::{json, Value};

/// Creates a new site for a user.
///
/// # Arguments
///
/// * `state` - A reference to the application state.
/// * `user_id` - The unique identifier of the user creating the site.
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
  site_type: SiteType,
  repo_url: Option<String>,
) -> Result<Value, StatusCode> {
  Ok(json! ({
    "data": {
      "verify": true
    }
  }))
}
