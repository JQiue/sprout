use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::uuid,
};
use sea_orm::{EntityTrait, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
  app::AppState,
  components::user::model::*,
  entitys::{prelude::User, user},
  response::StatusCode,
};

pub async fn user_register(
  state: &AppState,
  nickname: String,
  email: String,
  password: String,
) -> Result<Value, StatusCode> {
  if has_user(UserQueryBy::Email(email.clone()), &state.db).await {
    return Err(StatusCode::UserExist);
  }
  let hashed = if let Ok(hash) = argon2(password.as_bytes(), b"@QQ.wjq21") {
    hash
  } else {
    return Err(StatusCode::HashPasswordError);
  };
  let user_id = uuid(&helpers::uuid::Alphabet::DEFAULT, 8);
  let created_at = utc_now();
  let model = user::ActiveModel {
    user_id: Set(user_id),
    nickname: Set(nickname),
    email: Set(email),
    password: Set(hashed),
    status: Set(user::UserStatus::Deleted),
    created_at: Set(created_at),
    ..Default::default()
  };

  match User::insert(model).exec(&state.db).await {
    Ok(result) => Ok(json!({ "insert_id": result.last_insert_id })),
    Err(_err) => Err(StatusCode::DbError),
  }
}

pub async fn user_login(
  state: &AppState,
  email: String,
  password: String,
) -> Result<Value, StatusCode> {
  if !has_user(UserQueryBy::Email(email.clone()), &state.db).await {
    return Err(StatusCode::UserNotExist);
  }

  let model = if let Ok(model) = User::find()
    .column(user::Column::Password)
    .one(&state.db)
    .await
  {
    match model {
      Some(model) => model,
      None => return Err(StatusCode::UserNotExist),
    }
  } else {
    return Err(StatusCode::DbError);
  };

  let matches = if let Ok(matches) = verify_argon2(model.password, password.as_bytes()) {
    matches
  } else {
    return Err(StatusCode::PasswordError);
  };

  if matches {
    #[derive(Serialize, Deserialize)]
    struct Payload {
      user_id: String,
    }
    let token = jwt::sign(
      Payload {
        user_id: model.user_id,
      },
      "sprout".to_owned(),
      10,
    )
    .unwrap();
    Ok(json!({
      "token": token
    }))
  } else {
    Err(StatusCode::PasswordError)
  }
}
