use actix_web::web;

mod auth;
mod families;
mod roles;
mod users;
mod utils;

use auth::Router as AuthRouter;
use families::Router as FamiliesRouter;
use roles::Router as RolesRouter;
use users::Router as UsersRouter;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    FamiliesRouter::init(cfg);
    RolesRouter::init(cfg);
    UsersRouter::init(cfg);
    AuthRouter::init(cfg);
}
