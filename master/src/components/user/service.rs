use entity::user;
use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::{Value, json};

use crate::{app::AppState, components::user::model::*, error::AppError};

pub async fn generate_casual_user(state: &AppState) -> Result<Value, AppError> {
  let user_type = "casual";
  let nickname = format!("casual_{}", nanoid(&Alphabet::UPPER, 12));
  let email = format!("casual_@{}.com", nanoid(&Alphabet::UPPER, 12));
  let hashed = argon2(
    &nanoid(&Alphabet::DEFAULT, 8),
    &nanoid(&Alphabet::DEFAULT, 8),
  )
  .map_err(|_| AppError::HashPasswordError)?;
  let user_id = nanoid(&Alphabet::DEFAULT, 8);
  let token = jwt::sign(user_id.clone(), &state.login_token_key, 86400)?;
  user::ActiveModel {
    user_id: Set(user_id),
    nickname: Set(nickname),
    email: Set(email),
    password: Set(hashed),
    status: Set("active".to_string()),
    r#type: Set(user_type.to_string()),
    created_at: Set(utc_now()),
    ..Default::default()
  }
  .insert(&state.db)
  .await?;
  Ok(json!({ "token": token }))
}

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
) -> Result<Value, AppError> {
  if has_user(UserQueryBy::Email(email.clone()), &state.db).await? {
    return Err(AppError::UserExist);
  }
  let mut user_type = "normal";
  if is_first_user(&state.db).await? {
    user_type = "admin";
  }
  let hashed = argon2(&password, "@QQ.wjq21").map_err(|_| AppError::HashPasswordError)?;
  let user_id = nanoid(&helpers::uuid::Alphabet::DEFAULT, 8);
  let user = user::ActiveModel {
    user_id: Set(user_id),
    nickname: Set(nickname),
    email: Set(email),
    password: Set(hashed),
    status: Set("active".to_string()),
    r#type: Set(user_type.to_string()),
    created_at: Set(utc_now()),
    ..Default::default()
  }
  .insert(&state.db)
  .await?;
  Ok(json!({ "insert_id": user.id }))
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
) -> Result<Value, AppError> {
  if !has_user(UserQueryBy::Email(email.clone()), &state.db).await? {
    return Err(AppError::UserNotFound);
  }
  let user = get_user(UserQueryBy::Email(email), &state.db).await?;
  if verify_argon2(&user.password, &password)? {
    let token = jwt::sign(user.user_id, &state.login_token_key, 86400)?;
    Ok(json!({
      "token": token
    }))
  } else {
    Err(AppError::PasswordError)
  }
}

pub async fn get_user_info(state: &AppState, user_id: String) -> Result<Value, AppError> {
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
) -> Result<Value, AppError> {
  Err(AppError::NotImplemented)
}

pub async fn refresh_user_token(_state: &AppState, _user_id: String) -> Result<Value, AppError> {
  Err(AppError::NotImplemented)
}
