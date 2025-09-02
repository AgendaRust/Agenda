use sea_orm_migration::{prelude::*, schema::*};

// Assuming the User migration is in this path and defines a User entity.
use crate::m20250817_224457_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the "reminders" table.
        manager
            .create_table(
                Table::create()
                    .table(Reminder::Table)
                    .if_not_exists()
                    .col(pk_auto(Reminder::Id))
                    .col(integer(Reminder::UserId).not_null())
                    .col(string(Reminder::Name).not_null())
                    .col(string_null(Reminder::Category).not_null())
                    .col(timestamp(Reminder::DateEnd).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reminders-user_id")
                            .from(Reminder::Table, Reminder::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the "reminders" table.
        manager
            .drop_table(Table::drop().table(Reminder::Table).to_owned())
            .await
    }
}

/// Defines the identifiers for the `Reminder` table and its columns.
#[derive(DeriveIden)]
enum Reminder {
    Table,
    Id,
    //#[sea_orm(iden = "user_id")]
    UserId,
    Name,
    Category,
    DateEnd
}
