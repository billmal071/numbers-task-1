use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_actix_web::TracingLogger;
use config::Config;

mod handlers;
mod routes;
mod config;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = Config::from_env().expect("Failed to load environment variables!!!");
    
    info!("Starting server at http://{}:{}", "localhost", 8080);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .service(web::scope("/api").configure(routes::config_routes))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
