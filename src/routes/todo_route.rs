use serde::{Deserialize, Serialize};

use actix_web::{web, Error, HttpResponse};

#[derive(Deserialize, Serialize)]
pub struct Todo {
    id: Option<i64>,
    product_type: Option<String>,
    name: Option<String>,
}

pub fn test_func() {
    println!("test_func works");
}

pub fn get_todo(_query: String) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn add_todo(_new_product: web::Json<Todo>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn get_todo_detail(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub fn remove_todo(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

