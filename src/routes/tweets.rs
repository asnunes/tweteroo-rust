use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{protocols::MessageResponse, state};

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

        MessageResponse::new(StatusCode::CREATED, "OK!")
    } else {
        MessageResponse::new(StatusCode::UNAUTHORIZED, "User not found!")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetTweetResBody {
    tweet: String,
    username: String,
    avatar: String,
}

#[get("/tweets")]
pub async fn get_tweets(state: web::Data<state::TweterooState>) -> impl Responder {
    let tweets = state.get_tweets();
    let res_body = tweets
        .iter()
        .map(|tweet| GetTweetResBody {
            tweet: tweet.tweet.clone(),
            username: tweet.user.username.clone(),
            avatar: tweet.user.avatar.clone(),
        })
        .collect::<Vec<GetTweetResBody>>();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(res_body)
}
