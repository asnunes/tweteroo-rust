use actix_web::{http, HttpResponse, Responder};

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest};
use serde::Serialize;

pub struct MessageResponse<'a> {
    status: http::StatusCode,
    body: MessageBody<'a>,
}

#[derive(Serialize)]
struct MessageBody<'a> {
    message: &'a str,
}

impl<'a> Responder for MessageResponse<'a> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self.body).unwrap();

        HttpResponse::build(self.status)
            .insert_header(ContentType::json())
            .body(body)
    }
}

impl<'a> MessageResponse<'a> {
    pub fn new(status: http::StatusCode, message: &'a str) -> Self {
        Self {
            status,
            body: MessageBody { message },
        }
    }
}
