use actix_web::{http::StatusCode, post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::{protocols::Response, state};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTweetReqBody {
    username: String,
    tweet: String,
}

#[post("/tweets")]
pub async fn post_tweet(
    req_body: web::Json<PostTweetReqBody>,
    state: web::Data<state::TweterooState>,
) -> impl Responder {
    let user = state.get_user(&req_body.username);

    if let Some(user) = user {
        let tweet = state::Tweet::new(&user.id, &req_body.tweet);
        state.add_tweet(tweet);

        Response::new(StatusCode::CREATED, "OK!")
    } else {
        Response::new(StatusCode::UNAUTHORIZED, "User not found!")
    }
}
