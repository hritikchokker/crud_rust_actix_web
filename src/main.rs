mod constants;
mod controllers;
mod db;
mod errors;
mod models;
mod routes;
pub mod schema;
mod utils;
mod server;



fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let (domain, address, pool) = db::establish_connection();
    server::boot_http_server(pool, domain, address)?;
    Ok(())
}