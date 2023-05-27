use actix_web::{delete, error, get, post, put, web, Error, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use futures::StreamExt;

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    email: String,
    password: String,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

// POST /auth/login
#[post("/login")]
pub async fn login(mut login_info: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = login_info.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<LoginInfo>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
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
