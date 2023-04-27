use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{protocols::Message, state};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpReqBody {
    username: String,
    avatar: String,
}

#[post("/sign-up")]
pub async fn sign_up(
    req_body: web::Json<SignUpReqBody>,
    state: web::Data<state::TweterooState>,
) -> impl Responder {
    println!("body {:?}", req_body);

    let user = state::User::new(&req_body.username, &req_body.avatar);
    state.add_user(user);

    Message::new("OK!")
}
