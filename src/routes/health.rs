use actix_web::{get, Responder};

use crate::protocols::Message;

#[get("/health")]
pub async fn health() -> impl Responder {
    Message::new("OK!")
}
