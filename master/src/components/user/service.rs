use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::uuid,
};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{json, Value};

use crate::{
  app::AppState, components::user::model::*, entities::user, middleware::auth::JwtPayload,
  response::StatusCode,
};

/// Registers a new user in the system.
///
/// This function creates a new user account with the provided details. It checks if the email
/// is already in use, hashes the password, generates a unique user ID, and inserts the new user
/// into the database.
///
/// # Parameters
///
/// * `state` - A reference to the application state, which includes the database connection.
/// * `nickname` - The user's chosen nickname or display name.
/// * `email` - The user's email address, which must be unique in the system.
/// * `password` - The user's chosen password, which will be hashed before storage.
///
/// # Returns
///
/// Returns a `Result` which is:
/// * `Ok(Value)` - A JSON object containing the `insert_id` of the newly created user if successful.
/// * `Err(StatusCode)` - An error status code if registration fails. Possible errors include:
///   - `UserExist` if the email is already registered
///   - `HashPasswordError` if password hashing fails
///   - `DbError` if there's an issue with the database operation
pub async fn user_register(
  state: &AppState,
  nickname: String,
  email: String,
  password: String,
) -> Result<Value, StatusCode> {
  if has_user(UserQueryBy::Email(email.clone()), &state.db).await? {
    return Err(StatusCode::UserExist);
  }
  let mut user_type = UserType::Normal;
  if is_first_user(&state.db).await? {
    user_type = UserType::Admin;
  }
  let hashed =
    argon2(password.as_bytes(), b"@QQ.wjq21").map_err(|_| StatusCode::HashPasswordError)?;
  let user_id = uuid(&helpers::uuid::Alphabet::DEFAULT, 8);
  match (user::ActiveModel {
    user_id: Set(user_id),
    nickname: Set(nickname),
    email: Set(email),
    password: Set(hashed),
    status: Set(UserStatus::Active),
    r#type: Set(user_type),
    created_at: Set(utc_now()),
    ..Default::default()
  })
  .insert(&state.db)
  .await
  {
    Ok(result) => Ok(json!({ "insert_id": result.id })),
    Err(_err) => Err(StatusCode::DbError),
  }
}

/// This function verifies the user's credentials and generates a JWT token upon successful authentication.
///
/// # Parameters
///
/// * `state` - A reference to the application state, which includes the database connection.
/// * `email` - The email address of the user attempting to log in.
/// * `password` - The password provided by the user for authentication.
///
/// # Returns
///
/// Returns a `Result` which is:
/// * `Ok(Value)` - A JSON object containing the JWT token if authentication is successful.
/// * `Err(StatusCode)` - An error status code if authentication fails. Possible errors include:
///   - `UserNotExist` if the email is not registered
///   - `DbError` if there's an issue with the database operation
///   - `PasswordError` if the provided password is incorrect
pub async fn user_login(
  state: &AppState,
  email: String,
  password: String,
) -> Result<Value, StatusCode> {
  if !has_user(UserQueryBy::Email(email.clone()), &state.db).await? {
    return Err(StatusCode::UserNotFound);
  }
  let user = get_user(UserQueryBy::Email(email), &state.db).await?;
  if verify_argon2(user.password, password.as_bytes()).map_err(|_| StatusCode::ServerError)? {
    let token = jwt::sign(
      JwtPayload {
        user_id: user.user_id,
        user_type: user.r#type,
      },
      state.login_token_key.clone(),
      86400,
    )
    .map_err(|_| StatusCode::ServerError)?;
    Ok(json!({
      "token": token
    }))
  } else {
    Err(StatusCode::PasswordError)
  }
}

pub async fn get_user_info(state: &AppState, user_id: String) -> Result<Value, StatusCode> {
  let model = get_user(UserQueryBy::UserId(user_id), &state.db).await?;
  Ok(json!({
      "user_id": model.user_id,
      "nickname": model.nickname,
      "email": model.email,
      "status": model.status
  }))
}

pub async fn set_user_password(
  _state: &AppState,
  _user_id: String,
  _password: String,
) -> Result<Value, StatusCode> {
  Err(StatusCode::NotImplemented)
}

pub async fn refresh_user_token(_state: &AppState, _user_id: String) -> Result<Value, StatusCode> {
  Err(StatusCode::NotImplemented)
}
