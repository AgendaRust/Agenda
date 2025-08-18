use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(integer(Task::Id).not_null().auto_increment().primary_key())
                    .col(string(Task::Title).not_null())
                    .col(string(Task::Text).not_null())
                    .col(string(Task::Status).not_null())
                    .col(timestamp(Task::CompletionDate).not_null())
                    .col(
                        timestamp(Task::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Title,
    Text,
    Status,
    CompletionDate,
    CreatedAt,
}
