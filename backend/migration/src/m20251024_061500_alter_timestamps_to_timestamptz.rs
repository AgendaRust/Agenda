use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "task" 
                ALTER COLUMN begin_date TYPE TIMESTAMPTZ USING (begin_date AT TIME ZONE 'UTC'),
                ALTER COLUMN complete_date TYPE TIMESTAMPTZ USING (complete_date AT TIME ZONE 'UTC');
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "goal" 
                ALTER COLUMN date_start TYPE TIMESTAMPTZ USING (date_start AT TIME ZONE 'UTC'),
                ALTER COLUMN date_end TYPE TIMESTAMPTZ USING (date_end AT TIME ZONE 'UTC');
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "reminder" 
                ALTER COLUMN date_end TYPE TIMESTAMPTZ USING (date_end AT TIME ZONE 'UTC');
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "notes" 
                ALTER COLUMN created_at TYPE TIMESTAMPTZ USING (created_at AT TIME ZONE 'UTC');
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "task" 
                ALTER COLUMN begin_date TYPE TIMESTAMP,
                ALTER COLUMN complete_date TYPE TIMESTAMP;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "goal" 
                ALTER COLUMN date_start TYPE TIMESTAMP,
                ALTER COLUMN date_end TYPE TIMESTAMP;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "reminder" 
                ALTER COLUMN date_end TYPE TIMESTAMP;
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE "notes" 
                ALTER COLUMN created_at TYPE TIMESTAMP;
                "#,
            )
            .await?;

        Ok(())
    }
}
