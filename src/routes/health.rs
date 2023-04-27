use actix_web::{get, http::StatusCode, Responder};

use crate::protocols::MessageResponse;

#[get("/health")]
pub async fn health() -> impl Responder {
    MessageResponse::new(StatusCode::OK, "OK!")
}
