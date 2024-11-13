use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum User {
  Table,
  Id,
  UserId,
  Nickname,
  Password,
  Email,
  Type,
  Status,
  LastLoginAt,         // 最后登录时间
  LastLoginIp,         // 最后登录IP
  Avatar,              // 头像URL
  IsEmailVerified,     // 邮箱是否验证
  IsPhoneVerified,     // 手机是否验证
  FailedLoginAttempts, // 登录失败次数
  CreatedAt,
  UpdatedAt,
  DeletedAt, // 软删除时间
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(pk_auto(User::Id))
          .col(string(User::UserId).unique_key().comment("用户 UUID"))
          .col(string(User::Nickname))
          .col(string(User::Password))
          .col(string(User::Email))
          .col(string_null(User::Avatar).default("no"))
          .col(string(User::Type).default("normal"))
          .col(string(User::Status).default("active"))
          .col(tiny_integer(User::IsEmailVerified).default(0))
          .col(tiny_integer(User::IsPhoneVerified).default(0))
          .col(tiny_integer(User::FailedLoginAttempts).default(0))
          .col(string_null(User::LastLoginIp))
          .col(timestamp_null(User::LastLoginAt))
          .col(timestamp(User::CreatedAt))
          .col(timestamp_null(User::UpdatedAt))
          .col(timestamp_null(User::DeletedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(User::Table).to_owned())
      .await
  }
}
