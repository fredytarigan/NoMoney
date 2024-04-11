use actix_web::web;

mod families;

use families::RouteFamily;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    RouteFamily::init(cfg);
}
