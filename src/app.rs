use actix_web::{App, HttpServer,web};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;
mod routes;

use routes::init_routes;

// Function to establish a database connection pool
fn establish_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    // Load the DATABASE_URL from the .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection manager for SQLite
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");

    pool
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    // Initialize the database connection pool
    let pool = establish_connection_pool();

    // Start the Actix Web server and pass the database pool to the app's data
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass the database pool as app data
            .configure(init_routes)
        // ... add your routes and middleware ...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
