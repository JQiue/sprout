use sea_orm::EnumIter;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum User {
  Table,           // 表名
  Id,              // 主键 ID
  UserId,          // 用户 UUID
  Nickname,        // 昵称
  Password,        // 密码
  Email,           // 邮箱
  Type,            // casual, paying, admin
  Status,          // active, deleted
  IsEmailVerified, // 邮箱是否验证
  IsPhoneVerified, // 手机是否验证
  CreatedAt,       // 创建时间
  UpdatedAt,       // 更新时间
}

#[derive(Iden, EnumIter)]
pub enum UserType {
  #[iden = "casual"]
  Casual,
  #[iden = "paying"]
  Paying,
  #[iden = "admin"]
  Admin,
}

#[derive(Iden, EnumIter)]
pub enum UserStatus {
  #[iden = "active"]
  Active,
  #[iden = "deleted"]
  Deleted,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(pk_auto(User::Id).unsigned())
          .col(string(User::UserId).unique_key().comment("用户 UUID"))
          .col(string(User::Nickname).comment("昵称"))
          .col(string(User::Password).comment("密码"))
          .col(string(User::Email).comment("邮箱"))
          .col(
            string(User::Type)
              .default("casual")
              .comment("casual, paying, admin"),
          )
          .col(
            string(User::Status)
              .default("active")
              .comment("active, deleted"),
          )
          .col(
            tiny_integer(User::IsEmailVerified)
              .default(0)
              .comment("邮箱是否验证"),
          )
          .col(
            tiny_integer(User::IsPhoneVerified)
              .default(0)
              .comment("手机是否验证"),
          )
          .col(timestamp(User::CreatedAt).comment("创建时间"))
          .col(timestamp_null(User::UpdatedAt).comment("更新时间"))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(User::Table).to_owned())
      .await
  }
}
