use actix_web::{App, HttpServer};

mod routes;

use routes::init_routes;

pub struct AppState {
    // Define your state here (e.g., database connection pool)
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    // Create your state (e.g., initialize database connection pool)
    let state = AppState {
        // Initialize your state here
    };

    HttpServer::new(move || {
        App::new().configure(init_routes)

        // Pass the state to the application
        // .service(
        //     web::scope("/")
        //         .service(routes::todo_route::get_all_todo)
        //         .service(routes::todo_route::create_todo)
        //         .service(routes::todo_route::update_todo)
        //         .service(routes::todo_route::delete_todo)
        //         .service(routes::todo_route::get_todo_by_user), // Add more routes for other features here
        // )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
