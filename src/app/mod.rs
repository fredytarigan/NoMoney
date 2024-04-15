use actix_web::web;

mod auth;
mod families;
mod permissions;
mod response;
mod roles;
mod users;
mod utils;

// collection of /api/v1
use auth::Router as AuthRouter;
use families::Router as FamiliesRouter;
use roles::Router as RolesRouter;
use users::Router as UsersRouter;

pub trait RouterConfig {
    fn init(cfg: &mut web::ServiceConfig);
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    FamiliesRouter::init(cfg);
    RolesRouter::init(cfg);
    UsersRouter::init(cfg);
    AuthRouter::init(cfg);
}

pub use response::Response;
