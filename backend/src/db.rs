use rocket::fairing::{self, AdHoc};
use sea_orm::{Database, DatabaseConnection};

pub type Pool = DatabaseConnection;

// The name of the database connection in Rocket's config
const DB_NAME: &str = "my_db";

/// A helper function to fetch the database URL from the environment
fn db_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.")
}

/// An AdHoc fairing for initializing the SeaORM database connection
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

/// A request guard for retrieving a database connection from the pool
pub async fn get_conn<'r>(req: &'r rocket::Request<'_>) -> Result<Pool, ()> {
    req.rocket()
        .state::<Pool>()
        .map(|pool| pool.clone())
        .ok_or(())
}
