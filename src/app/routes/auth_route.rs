use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

// POST /auth/login
#[post("/login")]
pub async fn login() -> impl Responder {
    // Logic for login
    HttpResponse::Ok().body("Login")
}

// POST /auth/register
#[post("/register")]
pub async fn register() -> impl Responder {
    // Logic for register
    HttpResponse::Ok().body("Register")
}

// POST /auth/logout
#[post("/logout")]
pub async fn logout() -> impl Responder {
    // Logic for logout
    HttpResponse::Ok().body("Logout")
}
