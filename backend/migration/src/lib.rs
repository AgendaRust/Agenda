pub use sea_orm_migration::prelude::*;

mod m20250815_052738_create_tables;
// mod m20250816_012525_new_task_table;
mod m20250817_224457_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250815_052738_create_tables::Migration),
            Box::new(m20250817_224457_user::Migration),
        ]
    }
}
