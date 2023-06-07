pub mod index;

use crate::errors::ServiceError;
use easy_password::bcrypt::{hash_password, verify_password};

use actix_identity::{CookieIdentityPolicy, IdentityService};

use actix_web::{http::header};


use actix_cors::Cors;

lazy_static::lazy_static! {
pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash(password: &str) -> Result<String, ServiceError> {
    hash_password(password, SECRET_KEY.as_bytes(), 12).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    verify_password(password, hash, SECRET_KEY.as_bytes()).map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })
}

pub fn setup_indentity_service(domain: &String) -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(self::SECRET_KEY.as_bytes())
            .name("auth")
            .path("/")
            .domain(domain.as_str())
            .max_age_time(chrono::Duration::days(1))
            .secure(false), // this can only be true if you have https
    )
}



pub fn setup_cors_handler(address: &String) -> Cors {
    Cors::new()
        .allowed_origin(&address)
        .send_wildcard()
        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(3600)
}