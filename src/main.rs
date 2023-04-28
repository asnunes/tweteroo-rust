use actix_web::{web, App, HttpResponse, HttpServer};
use protocols::MessageBody;
use routes::{health, tweets, users};
use state::TweterooState;

mod protocols;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default().error_handler(|err, _req| {
            let err_str = format!("{}", err);

            actix_web::error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(MessageBody::new(&err_str)),
            )
            .into()
        });

        App::new()
            .app_data(web::Data::new(TweterooState::new()))
            .app_data(json_config)
            .service(health::health)
            .service(tweets::post_tweet)
            .service(tweets::get_tweets)
            .service(users::sign_up)
    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
