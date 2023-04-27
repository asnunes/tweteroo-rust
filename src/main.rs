use actix_web::{web, App, HttpServer};
use state::TweterooState;

mod routes;
mod state;

use crate::routes::health::health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(TweterooState::new()))
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
