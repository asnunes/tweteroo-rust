use actix_web::{http::StatusCode, post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::{protocols::Response, state};

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
    let user = state::User::new(&req_body.username, &req_body.avatar);
    state.add_user(user);

    Response::new(StatusCode::CREATED, "OK!")
}
