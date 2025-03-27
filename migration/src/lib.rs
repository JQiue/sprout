pub use sea_orm_migration::prelude::*;

mod create_table_agent;
mod create_table_deployment;
mod create_table_nginx;
mod create_table_site;
mod create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(create_table_user::Migration),
      Box::new(create_table_agent::Migration),
      Box::new(create_table_site::Migration),
      Box::new(create_table_nginx::Migration),
      Box::new(create_table_deployment::Migration),
    ]
  }
}
