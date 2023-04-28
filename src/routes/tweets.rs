use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    protocols::{MessageBody, MessageResponse},
    state,
};

pub fn scope() -> actix_web::Scope {
    web::scope("/tweets")
        .route("/", web::post().to(post_tweet))
        .route("/", web::get().to(get_tweets))
        .route("/{username}", web::get().to(get_tweets_by_username))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTweetReqBody {
    username: String,
    tweet: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTweetQuery {
    page: Option<usize>,
}

pub async fn get_tweets(
    state: web::Data<state::TweterooState>,
    query: web::Query<GetTweetQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    if page < 1 {
        return HttpResponse::BadRequest().json(MessageBody::new("Page must be greater than 0!"));
    }

    let tweets = state.get_tweets(page);
    let res_body = tweets
        .iter()
        .map(|tweet| GetTweetResBody {
            tweet: tweet.tweet.clone(),
            username: tweet.user.username.clone(),
            avatar: tweet.user.avatar.clone(),
        })
        .collect::<Vec<GetTweetResBody>>();

    HttpResponse::Ok().json(res_body)
}

pub async fn get_tweets_by_username(
    path: web::Path<(String,)>,
    state: web::Data<state::TweterooState>,
) -> impl Responder {
    let tweets = state.get_tweets_by_username(&path.0);
    let res_body = tweets
        .iter()
        .map(|tweet| GetTweetResBody {
            tweet: tweet.tweet.clone(),
            username: tweet.user.username.clone(),
            avatar: tweet.user.avatar.clone(),
        })
        .collect::<Vec<GetTweetResBody>>();

    HttpResponse::Ok().json(res_body)
}
