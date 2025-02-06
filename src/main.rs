use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{App, HttpServer};

mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
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
            .wrap(cors)
            .configure(routes::config_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
