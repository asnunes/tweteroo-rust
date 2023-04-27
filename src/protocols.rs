use actix_web::{http, HttpResponse, Responder};

use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest};
use serde::Serialize;

pub struct Message<'a> {
    status: http::StatusCode,
    body: MessageBody<'a>,
}

#[derive(Serialize)]
struct MessageBody<'a> {
    message: &'a str,
}

impl<'a> Responder for Message<'a> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self.body).unwrap();

        HttpResponse::build(self.status)
            .insert_header(ContentType::json())
            .body(body.into())
    }
}

impl<'a> Message<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}
