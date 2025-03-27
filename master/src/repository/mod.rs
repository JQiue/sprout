mod user;

use sea_orm::DatabaseConnection;

pub use user::UserRepository;

#[derive(Debug, Clone)]
pub struct RepositoryManager {
  pub db: DatabaseConnection,
}

impl RepositoryManager {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db }
  }

  pub fn user(&self) -> UserRepository {
    UserRepository { db: &self.db }
  }
}
