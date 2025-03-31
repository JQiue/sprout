mod agent;
mod deployment;
mod site;
mod user;

use deployment::DeploymentRepository;
use sea_orm::DatabaseConnection;

pub use agent::AgentRepository;
pub use site::SiteRepository;
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
  pub fn site(&self) -> SiteRepository {
    SiteRepository { db: &self.db }
  }
  pub fn agent(&self) -> AgentRepository {
    AgentRepository { db: &self.db }
  }

  pub fn deployment(&self) -> DeploymentRepository {
    DeploymentRepository { db: &self.db }
  }
}
