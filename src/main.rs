use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use protocols::MessageBody;
use routes::{health, tweets, users};
use state::TweterooState;

mod protocols;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
            .wrap(Logger::new("%r %s \"%{User-Agent}i\" %T"))
            .app_data(web::Data::new(TweterooState::new()))
            .app_data(json_config)
            .service(health::health)
            .service(users::sign_up)
            .service(tweets::scope())
    })
    .workers(1)
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
