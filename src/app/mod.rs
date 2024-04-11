use actix_web::web;

mod families;
mod roles;

use families::RouteFamily;
use roles::RouteRoles;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    RouteFamily::init(cfg);
    RouteRoles::init(cfg);
}
