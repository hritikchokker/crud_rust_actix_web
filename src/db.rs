use crate::models;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
// mod models;
pub fn establish_connection() -> (String, String, models::Pool) {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let address: String =
        std::env::var("ADDRESS").unwrap_or_else(|_| "http://localhost".to_string());

    // create db connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    (domain, address, pool)
}
