use actix_web::web;

mod families;
mod roles;

use families::RouteFamilies;
use roles::RouteRoles;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    RouteFamilies::init(cfg);
    RouteRoles::init(cfg);
}
