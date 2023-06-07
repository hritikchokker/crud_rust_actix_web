// use crate::constants::index::constants_func;
use crate::controllers::user_controller::user_controller_fn;
// use crate::controllers::index::controllers_func;
// use crate::models::index::models_func;
use crate::routes::{configure_app_routes};
use crate::utils::{setup_cors_handler, setup_indentity_service};

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod constants;
mod controllers;
mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod utils;

fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("pong !!")
}


fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    user_controller_fn();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    let (domain, address, pool) = db::establish_connection();
    // establish_connection();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(setup_indentity_service(&domain))
            .data(web::JsonConfig::default().limit(4096))
            .wrap(setup_cors_handler(&address))
            .service(web::resource("/ping").to(ping))
            .service(
                web::scope("/api")
                .configure(configure_app_routes)
            )
    })
    .bind("0.0.0.0:8000")
    .expect("Cannot bind to 0.0.0.0:8000")
    .workers(1)
    .run()
}
