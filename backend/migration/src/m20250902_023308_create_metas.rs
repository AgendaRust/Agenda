use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250817_224457_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .create_table(
                Table::create()
                    .table(Goals::Table)
                    .if_not_exists()
                    .col(pk_auto(Goals::Id))
                    .col(integer(Goals::User_id))
                    .col(string(Goals::Name))
                    .col(string_null(Goals::Description))
                    .col(string_null(Goals::Category))
                    .col(string(Goals::Status))
                    .col(string(Goals::Type))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-goals-user_id")
                            .from(Goals::Table, Goals::User_id)
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
            .drop_table(Table::drop().table(Goals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Goals {
    Table,
    Id,
    User_id,
    Name,
    Description,
    Category,
    Status,
    Type,
}
