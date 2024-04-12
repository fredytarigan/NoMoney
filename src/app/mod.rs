use actix_web::web;

mod families;
mod roles;
mod users;
mod utils;

use families::RouteFamilies;
use roles::RouteRoles;
use users::Router as UsersRouter;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    RouteFamilies::init(cfg);
    RouteRoles::init(cfg);
    UsersRouter::init(cfg);
}
