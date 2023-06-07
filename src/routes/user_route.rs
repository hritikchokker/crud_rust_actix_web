use serde::{Deserialize, Serialize};

use actix_web::{web, Error, HttpResponse};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: Option<i64>,
    product_type: Option<String>,
    name: Option<String>,
}

pub fn test_func() {
    println!("test_func works");
}

pub fn get_user(_query: String) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn add_user(_new_product: web::Json<User>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn get_user_detail(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn remove_user(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}