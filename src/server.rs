use crate::routes::configure_app_routes;
use crate::utils::{setup_cors_handler, setup_indentity_service};
// use crate::controllers::user_controller::user_controller_fn;
// use crate::constants::index::constants_func;
// use crate::models::index::models_func;
// use crate::controllers::index::controllers_func;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use crate::models;


pub fn ping(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("pong !!")
}

pub fn boot_http_server(
    pool: models::Pool,
    domain: String,
    address: String,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(setup_indentity_service(&domain))
            .data(web::JsonConfig::default().limit(4096))
            .wrap(setup_cors_handler(&address))
            .service(web::resource("/ping").to(ping))
            .service(web::scope("/api").configure(configure_app_routes))
    })
    .bind("0.0.0.0:8000")?
    .workers(1)
    .run()?;
    Ok(())
}
