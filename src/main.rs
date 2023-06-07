// use crate::constants::index::constants_func;
use crate::controllers::user_controller::user_controller_fn;
// use crate::controllers::index::controllers_func;
// use crate::models::index::models_func;
// use crate::routes::index::routes_func;
// use crate::utils::index::utils_func;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    http::header, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

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

fn indentity_service(domain: &String) -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
            .name("auth")
            .path("/")
            .domain(domain.as_str())
            .max_age_time(chrono::Duration::days(1))
            .secure(false), // this can only be true if you have https
    )
}

fn cors_handler(address: &String) -> Cors {
    Cors::new()
        .allowed_origin(&address)
        .send_wildcard()
        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(3600)
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
            .wrap(indentity_service(&domain))
            .data(web::JsonConfig::default().limit(4096))
            .wrap(cors_handler(&address))
            .service(web::resource("/ping").to(ping))
    })
    .bind("0.0.0.0:8000")
    .expect("Cannot bind to 0.0.0.0:8000")
    .workers(1)
    .run()
}
