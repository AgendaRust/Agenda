pub use sea_orm_migration::prelude::*;

mod m20250815_052738_create_tables;
// mod m20250816_012525_new_task_table;
mod m20250817_224457_user;
mod m20250818_233038_task;
mod m20250902_023308_create_metas;
mod m20250902_024553_create_reminder;
mod m20251024_061500_alter_timestamps_to_timestamptz;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250815_052738_create_tables::Migration),
            Box::new(m20250817_224457_user::Migration),
            Box::new(m20250818_233038_task::Migration),
            Box::new(m20250902_023308_create_metas::Migration),
            Box::new(m20250902_024553_create_reminder::Migration),
            Box::new(m20251024_061500_alter_timestamps_to_timestamptz::Migration),
        ]
    }
}
