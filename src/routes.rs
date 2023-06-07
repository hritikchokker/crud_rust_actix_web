use actix_web::web;
use todo_route::{add_todo, get_todo, get_todo_detail, remove_todo};
use user_route::{add_user, get_user, get_user_detail, remove_user};

pub mod index;
mod todo_route;
mod user_route;

fn configure_todo_routes(cnf: &mut web::ServiceConfig) {
    cnf.service(
        web::scope("/todo")
            .service(
                web::resource("")
                    .route(web::get().to(get_todo))
                    .route(web::post().to(add_todo)),
            )
            .service(
                web::scope("/{product_id}").service(
                    web::resource("")
                        .route(web::get().to(get_todo_detail))
                        .route(web::delete().to(remove_todo)),
                ),
            ),
    );
}

fn configure_user_route(cnf: &mut web::ServiceConfig){
    cnf.service(
        web::scope("/user")
            .service(
                web::resource("")
                    .route(web::get().to(get_user))
                    .route(web::post().to(add_user)),
            )
            .service(
                web::scope("/{product_id}").service(
                    web::resource("")
                        .route(web::get().to(get_user_detail))
                        .route(web::delete().to(remove_user)),
                ),
            ),
    );
}

pub fn configure_app_routes(cnf: &mut web::ServiceConfig) {
    configure_todo_routes(cnf);
    configure_user_route(cnf);
}
