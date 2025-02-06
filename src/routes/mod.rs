use actix_web::web;

mod route;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(route::get_health_status);
    cfg.service(route::classify_number);
}
