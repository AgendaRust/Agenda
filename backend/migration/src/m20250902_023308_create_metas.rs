use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250817_224457_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Goal::Table)
                    .if_not_exists()
                    .col(pk_auto(Goal::Id))
                    .col(integer(Goal::UserId).not_null())
                    .col(string(Goal::Name).not_null())
                    .col(string_null(Goal::Description))
                    .col(string_null(Goal::Category))
                    .col(string(Goal::Status))
                    .col(string(Goal::Type))
                    .col(timestamp(Goal::DateEnd))
                    .col(timestamp(Goal::DateStart))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-goal-user_id")
                            .from(Goal::Table, Goal::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Goal::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Goal {
    Table,
    Id,
    UserId,
    DateEnd,
    DateStart,
    Name,
    Description,
    Category,
    Status,
    Type,
}
