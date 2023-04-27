use actix_web::{web, App, HttpServer};
use routes::{health, tweets, users};
use state::TweterooState;

mod protocols;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(TweterooState::new()))
            .service(health::health)
            .service(tweets::post_tweet)
            .service(users::sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
