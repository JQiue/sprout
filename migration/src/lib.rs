pub use sea_orm_migration::prelude::*;

mod create_table_deployment;
mod create_table_project;
mod create_table_server;
mod create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(create_table_user::Migration),
      Box::new(create_table_server::Migration),
      Box::new(create_table_project::Migration),
    ]
  }
}
