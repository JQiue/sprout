use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Nginx {
  Table,
  Id,            // 主键 ID
  SiteId,        // 关联的站点 ID
  AgentId,       // 关联的 AgentID
  ConfigContent, // Nginx 配置内容
  CreatedAt,     // 创建时间
  UpdatedAt,     // 更新时间
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Nginx::Table)
          .if_not_exists()
          .col(pk_auto(Nginx::Id))
          .col(string(Nginx::SiteId).unique_key().comment("站点 ID"))
          .col(string(Nginx::AgentId).unique_key().comment("Agent ID"))
          .col(string(Nginx::ConfigContent).comment("Nginx配置内容"))
          .col(timestamp(Nginx::CreatedAt).comment("创建时间"))
          .col(timestamp_null(Nginx::UpdatedAt).comment("更新时间"))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Nginx::Table).to_owned())
      .await
  }
}
