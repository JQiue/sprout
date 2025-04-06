use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::user::{self, UserType};

pub enum UserQueryBy<'a> {
  UserId(String),
  Email(&'a str),
}

#[derive(Debug, Clone)]
pub struct UserRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
  // pub async fn get_users(&self) -> Result<Vec<user::Model>, DbErr> {
  //   user::Entity::find().all(self.db).await
  // }

  pub async fn get_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
    self.get_user(UserQueryBy::Email(email)).await
  }

  pub async fn get_user_by_id(&self, id: String) -> Result<Option<user::Model>, DbErr> {
    self.get_user(UserQueryBy::UserId(id)).await
  }

  async fn get_user(&self, query_by: UserQueryBy<'a>) -> Result<Option<user::Model>, DbErr> {
    let mut select = user::Entity::find();
    match query_by {
      UserQueryBy::UserId(id) => select = select.filter(user::Column::UserId.eq(id)),
      UserQueryBy::Email(email) => select = select.filter(user::Column::Email.eq(email)),
    }
    select.one(self.db).await
  }

  pub async fn is_admin_user(&self, user_id: &str) -> Result<bool, DbErr> {
    let user = user::Entity::find()
      .filter(user::Column::UserId.eq(user_id))
      .one(self.db)
      .await?;
    match user {
      Some(user) => Ok(user.r#type == UserType::Administrator),
      None => Ok(false),
    }
  }

  pub async fn create_user(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
    user.insert(self.db).await
  }

  // pub async fn update_user(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
  //   user.update(self.db).await
  // }

  // pub async fn has_user_by_id(&self, id: String) -> Result<bool, DbErr> {
  //   self.has_user(UserQueryBy::UserId(id)).await
  // }

  pub async fn has_user_by_email(&self, email: &str) -> Result<bool, DbErr> {
    self.has_user(UserQueryBy::Email(email)).await
  }

  pub async fn has_user(&self, query_by: UserQueryBy<'a>) -> Result<bool, DbErr> {
    let mut select = user::Entity::find();
    match query_by {
      UserQueryBy::UserId(id) => select = select.filter(user::Column::UserId.eq(id)),
      UserQueryBy::Email(email) => select = select.filter(user::Column::Email.eq(email)),
    }
    let res = select.one(self.db).await?;
    Ok(res.is_some())
  }

  pub async fn is_first_user(&self) -> Result<bool, DbErr> {
    let users = user::Entity::find().all(self.db).await?;
    Ok(users.is_empty())
  }
}
