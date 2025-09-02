use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(string(Task::Description))
                    .col(string(Task::Status).not_null()) //put enum here
                    .col(timestamp(Task::BeginDate).not_null())
                    .col(timestamp(Task::CompleteDate).not_null())
                    .col(string(Task::Category).not_null())
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

// id -> int
// user_id -> int
// nome -> string
// data-inicio -> data (dia) after now
// data-fim ->
// categoria -> string
// status -> string (executada, pendente, totalmente)
// tipo -> (meia-hora/uma-hora/manha/tarde/noite)

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Title,
    Description,
    BeginDate,
    CompleteDate,
    Status,
    Category
}
