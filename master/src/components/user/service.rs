use entity::user::{self, UserStatus, UserType};
use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use sea_orm::Set;
use serde_json::{Value, json};

use crate::{app::AppState, error::AppError};

pub async fn generate_casual_user(state: &AppState) -> Result<Value, AppError> {
  let nickname = format!("casual_{}", nanoid(&Alphabet::UPPER, 12));
  let email = format!("casual_@{}.com", nanoid(&Alphabet::UPPER, 12));
  let hashed = argon2(
    &nanoid(&Alphabet::DEFAULT, 8),
    &nanoid(&Alphabet::DEFAULT, 8),
  )?;
  let user_id = nanoid(&Alphabet::DEFAULT, 8);
  let token = jwt::sign(user_id.clone(), &state.login_token_key, 86400)?;
  let active_user = user::ActiveModel {
    user_id: Set(user_id),
    nickname: Set(nickname),
    email: Set(email),
    password: Set(hashed),
    status: Set(UserStatus::Active),
    r#type: Set(UserType::Casual),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  state.repo.user().create_user(active_user).await?;
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
  if state.repo.user().has_user_by_email(&email).await? {
    return Err(AppError::UserExists);
  }
  let mut user_type = UserType::Normal;
  if state.repo.user().is_first_user().await? {
    user_type = UserType::Administrator;
  }
  let hashed = argon2(&password, &nanoid(&Alphabet::DEFAULT, 8))?;
  let user_id = nanoid(&Alphabet::DEFAULT, 8);
  let active_user = user::ActiveModel {
    user_id: Set(user_id),
    nickname: Set(nickname),
    email: Set(email),
    password: Set(hashed),
    status: Set(UserStatus::Active),
    r#type: Set(user_type),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  state.repo.user().create_user(active_user).await?;
  Ok(Value::Null)
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
  if let Some(user) = state.repo.user().get_user_by_email(&email).await? {
    if verify_argon2(&user.password, &password)? {
      let token = jwt::sign(user.user_id, &state.login_token_key, 86400)?;
      Ok(json!({
        "token": token
      }))
    } else {
      Err(AppError::PasswordError)
    }
  } else {
    Err(AppError::UserNotFound)
  }
}

pub async fn get_user_info(state: &AppState, user_id: String) -> Result<Value, AppError> {
  if let Some(user) = state.repo.user().get_user_by_id(user_id).await? {
    Ok(json!({
        "user_id": user.user_id,
        "nickname": user.nickname,
        "email": user.email,
        "status": user.status
    }))
  } else {
    Err(AppError::UserNotFound)
  }
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
