use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{protocols::MessageResponse, state};

pub fn scope() -> actix_web::Scope {
    web::scope("/tweets")
        .route("", web::post().to(post_tweet))
        .route("", web::get().to(get_tweets))
        .route("/{username}", web::get().to(get_tweets_by_username))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTweetReqBody {
    tweet: String,
}

pub async fn post_tweet(
    req_body: web::Json<PostTweetReqBody>,
    req: HttpRequest,
    state: web::Data<state::TweterooState>,
) -> impl Responder {
    let user_header = req.headers().get("user");
    if user_header.is_none() {
        return MessageResponse::new(StatusCode::BAD_REQUEST, "User not found!");
    }

    let username = user_header.unwrap().to_str().unwrap();
    let user = state.get_user(username);

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
