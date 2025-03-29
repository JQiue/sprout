use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::user;

pub enum UserQueryBy<'a> {
  Id(u32),
  Email(&'a str),
}

#[derive(Debug, Clone)]
pub struct UserRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
  pub async fn get_users(&self) -> Result<Vec<user::Model>, DbErr> {
    user::Entity::find().all(self.db).await
  }

  pub async fn get_user_by_email(&self, email: &str) -> Result<Option<user::Model>, DbErr> {
    self.get_user(UserQueryBy::Email(email)).await
  }

  pub async fn get_user_by_id(&self, id: u32) -> Result<Option<user::Model>, DbErr> {
    self.get_user(UserQueryBy::Id(id)).await
  }

  async fn get_user(&self, query_by: UserQueryBy<'a>) -> Result<Option<user::Model>, DbErr> {
    let mut select = user::Entity::find();
    match query_by {
      UserQueryBy::Id(id) => select = select.filter(user::Column::Id.eq(id)),
      UserQueryBy::Email(email) => select = select.filter(user::Column::Email.eq(email)),
    }
    select.one(self.db).await
  }

  pub async fn is_admin_user(&self, email: &str) -> Result<bool, DbErr> {
    let user = user::Entity::find()
      .filter(user::Column::Email.eq(email))
      .one(self.db)
      .await?;
    match user {
      Some(user) => Ok(user.r#type == "administrator"),
      None => Ok(false),
    }
  }

  pub async fn create_user(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
    user.insert(self.db).await
  }

  pub async fn update_user(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
    user.update(self.db).await
  }

  pub async fn set_2fa(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
    user.update(self.db).await
  }

  pub async fn has_user_by_id(&self, id: u32) -> Result<bool, DbErr> {
    self.has_user(UserQueryBy::Id(id)).await
  }

  pub async fn has_user_by_email(&self, email: &str) -> Result<bool, DbErr> {
    self.has_user(UserQueryBy::Email(email)).await
  }

  async fn has_user(&self, query_by: UserQueryBy<'a>) -> Result<bool, DbErr> {
    let mut select = user::Entity::find();
    match query_by {
      UserQueryBy::Id(id) => select = select.filter(user::Column::Id.eq(id)),
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
