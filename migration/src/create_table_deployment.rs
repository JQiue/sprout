use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Deployment {
  Table,
  Id,
  SiteId,  // 关联的站点ID
  AgentId, // 执行部署的AgentID
  Version,
  Status,           // 部署状态：pending, building, published, failed
  StartedAt,        // 开始时间
  CompletedAt,      // 完成时间
  CommitHash,       // Git提交hash
  CommitMessage,    // 提交信息
  Branch,           // 构建分支
  BuildLogs,        // 构建日志
  ErrorMessage,     // 错误信息
  DeployPreviewUrl, // 部署预览 URL
  BuildDuration,    // 构建时间
  DeployedAt,
  CreatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Deployment::Table)
          .if_not_exists()
          .col(pk_auto(Deployment::Id))
          .col(string(Deployment::SiteId))
          .col(string(Deployment::AgentId))
          .col(string(Deployment::Version))
          .col(string(Deployment::Status))
          .col(string(Deployment::CommitHash))
          .col(string(Deployment::CommitMessage))
          .col(string(Deployment::Branch))
          .col(string(Deployment::BuildLogs))
          .col(string(Deployment::ErrorMessage))
          .col(string(Deployment::DeployPreviewUrl))
          .col(string(Deployment::BuildDuration))
          .col(timestamp(Deployment::StartedAt))
          .col(timestamp(Deployment::CompletedAt))
          .col(timestamp(Deployment::DeployedAt))
          .col(timestamp(Deployment::CreatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Deployment::Table).to_owned())
      .await
  }
}
