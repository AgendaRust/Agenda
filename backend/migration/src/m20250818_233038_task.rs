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
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Title).not_null())
                    .col(integer(Task::UserId).not_null())
                    .col(string(Task::Description))
                    .col(string(Task::Status).not_null())
                    .col(timestamp(Task::BeginDate).not_null())
                    .col(timestamp(Task::CompleteDate).not_null())
                    .col(string(Task::Category).not_null())
                    .col(string(Task::Type).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-user_id")
                            .from(Task::Table, Task::UserId)
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
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

// data-inicio -> data (dia) after now
// status -> string (executada, pendente, totalmente)
// tipo -> (meia-hora/uma-hora/manha/tarde/noite)

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    UserId,
    Title,
    Description,
    BeginDate,
    CompleteDate,
    Status,
    Category,
    Type,
}
