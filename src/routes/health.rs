use actix_web::{get, HttpResponse, Responder};

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest};
use serde::Serialize;

#[derive(Serialize)]
struct Health<'a> {
    message: &'a str,
}

impl<'a> Responder for Health<'a> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/health")]
pub async fn health() -> impl Responder {
    Health { message: "OK!" }
}
