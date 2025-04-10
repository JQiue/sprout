use sea_orm::EnumIter;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Site {
  Table,
  Id,           // 主键 ID
  SiteId,       // 站点 ID
  UserId,       // 用户 ID
  DeploymentId, // 部署 ID
  Name,         // 站点名称
  Domain,       // 绑定域名
  Bandwidth,    // 带宽
  Status,       // 状态：active，disabled
  CreatedAt,    // 创建时间
  UpdatedAt,    // 更新时间
}

#[derive(Iden, EnumIter)]
pub enum SiteStatus {
  #[iden = "active"]
  Active,
  #[iden = "disabled"]
  Disabled,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Site::Table)
          .if_not_exists()
          .col(pk_auto(Site::Id).unsigned())
          .col(string(Site::SiteId).comment("站点ID"))
          .col(string(Site::UserId).comment("用户ID"))
          .col(
            integer_null(Site::DeploymentId)
              .unsigned()
              .comment("部署 ID"),
          )
          .col(string(Site::Name).comment("站点名称"))
          .col(string_null(Site::Domain).comment("绑定域名"))
          .col(integer_null(Site::Bandwidth).unsigned().comment("带宽"))
          .col(string(Site::Status).default("active").comment("状态"))
          .col(timestamp(Site::CreatedAt).comment("创建时间"))
          .col(timestamp_null(Site::UpdatedAt).comment("更新时间"))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Site::Table).to_owned())
      .await
  }
}
