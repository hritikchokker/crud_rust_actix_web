use actix_web::{delete, get, post, put, web, HttpResponse, Responder};


// PUT /todo/{id}
#[put("/todo/{id}")]
pub async fn update_todo(path: web::Path<(u32,)>) -> impl Responder {
    let todo_id = path.0;
    // Logic to update a todo with the given ID
    HttpResponse::Ok().body(format!("Update todo with ID: {}", todo_id))
}

// DELETE /todo/{id}
#[delete("/todo/{id}")]
pub async fn delete_todo(path: web::Path<(u32,)>) -> impl Responder {
    let todo_id = path.0;
    // Logic to delete a todo with the given ID
    HttpResponse::Ok().body(format!("Delete todo with ID: {}", todo_id))
}

// GET /todo/user/{user_id}
#[get("/todo/user/{user_id}")]
pub async fn get_user_todos(path: web::Path<(u32,)>) -> impl Responder {
    let user_id = path.0;
    // Logic to fetch todos for a specific user
    HttpResponse::Ok().body(format!("Get todos for user with ID: {}", user_id))
}


// GET /todo
#[get("/todo")]
pub async  fn get_all_todos() -> impl Responder {
    // Logic to fetch all todos
    HttpResponse::Ok().body("Get all todos")
}

// POST /todo
#[post("/todo")]
pub async fn create_todo() -> impl Responder {
    // Logic to create a new todo
    HttpResponse::Ok().body("Create todo")
}


