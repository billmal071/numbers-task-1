use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::filter::EnvFilter;

mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
