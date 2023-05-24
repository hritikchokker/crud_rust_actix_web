use actix_web::web;

pub mod todo_route;

pub mod auth_route;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(todo_route::get_all_todos)
            .service(todo_route::create_todo)
            .service(todo_route::update_todo)
            .service(todo_route::delete_todo)
            .service(todo_route::get_user_todos)
            .service(auth_route::login)
            .service(auth_route::register)
            .service(auth_route::logout),
    );
}
