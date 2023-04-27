use actix_web::{get, http::StatusCode, Responder};

use crate::protocols::Response;

#[get("/health")]
pub async fn health() -> impl Responder {
    Response::new(StatusCode::OK, "OK!")
}
