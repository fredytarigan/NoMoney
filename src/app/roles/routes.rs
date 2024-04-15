use super::models::SearchRole;
use super::repositories::Repository;
use crate::app::Response;
use crate::app::{permissions::AdminUser, utils::parse_uuid};
use crate::database::DbPool;
use crate::errors::ApplicationError;
use actix_web::{
    get,
    web::{self},
    HttpResponse, Result,
};
use serde_json::json;

pub struct Router;

impl Router {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/roles")
                .service(index_roles)
                .service(search_roles_by_name)
                .service(view_roles),
        );
    }
}

#[get("")]
async fn index_roles(
    db: web::Data<DbPool>,
    _user: AdminUser,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db.get().await?;

    let roles = Repository::find_all(&mut conn, 100).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("list of roles"),
        Some(json!(roles)),
        None,
    )
    .return_ok())
}

#[get("/{role_id}")]
async fn view_roles(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    _user: AdminUser,
) -> Result<HttpResponse, ApplicationError> {
    let role_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = parse_uuid(&role_id)?;

    let mut conn = db.get().await?;

    let roles = Repository::find_by_id(&mut conn, uid).await?;

    Ok(Response::new(
        200,
        2000,
        String::from("get roles"),
        Some(json!(roles)),
        None,
    )
    .return_ok())
}

#[get("/search")]
async fn search_roles_by_name(
    db: web::Data<DbPool>,
    query: web::Query<SearchRole>,
    _user: AdminUser,
) -> Result<HttpResponse, ApplicationError> {
    match query.name.to_owned() {
        Some(name) => {
            let mut conn = db.get().await?;
            let roles = Repository::find_by_name(&mut conn, &name).await?;

            Ok(Response::new(
                200,
                2000,
                String::from("get roles by name"),
                Some(json!(roles)),
                None,
            )
            .return_ok())
        }
        None => {
            info!("Missing query parameters for \"name\" field");

            let response = Response::new(
                400,
                4000,
                String::from("missing query parameters for 'name' field"),
                None,
                Some(json!(["invalid input"])),
            );

            Err(ApplicationError::new(response))
        }
    }
}
