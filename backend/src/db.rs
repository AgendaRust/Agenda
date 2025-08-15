use rocket::fairing::{self, AdHoc};
use sea_orm::{Database, DatabaseConnection};

use sea_orm_migration::prelude::*;

pub type Pool = DatabaseConnection;

const DB_NAME: &str = "my_db";

fn db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.")
}

pub fn init_pool() -> AdHoc {
    AdHoc::on_ignite("SeaORM Database Pool", |rocket| async {
        let pool = match Database::connect(db_url()).await {
            Ok(pool) => pool,
            Err(e) => {
                panic!("Failed to connect to database: {}", e);
            }
        };
        rocket.manage(pool)
    })
}

pub async fn get_conn<'r>(req: &'r rocket::Request<'_>) -> Result<Pool, ()> {
    req.rocket()
        .state::<Pool>()
        .map(|pool| pool.clone())
        .ok_or(())
}
