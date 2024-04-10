use crate::{
    errors::ApplicationError, models::Family, models::NewFamily, repositories::FamilyRepository,
    routes::DbPool,
};
use actix_web::{
    delete, error, get,
    http::StatusCode,
    post, put,
    web::{self},
    HttpResponse, Responder, Result,
};
use diesel::result::Error::NotFound;
use serde_json::json;
use uuid::Uuid;

pub struct RouteFamily;

impl RouteFamily {
    pub fn route(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/families")
                .service(index)
                .service(view_family)
                .service(create_family)
                .service(update_family)
                .service(delete_family),
        );
    }
}

#[get("")]
async fn index(db: web::Data<DbPool>) -> Result<impl Responder> {
    let mut conn = db
        .get()
        .await
        .expect("Failed to get connection from DB Pool");

    FamilyRepository::find_all(&mut conn, 100)
        .await
        .map(|family| {
            HttpResponse::Ok().json(json!(
                {
                    "result": "ok",
                    "data": family
                }
            ))
        })
        .map_err(|err| error::ErrorInternalServerError(err))
}

#[get("/{family_id}")]
async fn view_family(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = match Uuid::parse_str(&family_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError {
                message: format!("user {:?} not found", family_id.to_string()),
                status: 404,
            })
        }
    };

    let mut conn = db
        .get()
        .await
        .expect("Failed to get connection from DB Pool");

    FamilyRepository::find_by_id(&mut conn, uid)
        .await
        .map(|family| {
            HttpResponse::Ok().json(json!(
                {
                    "result": "ok",
                    "data": family
                }
            ))
        })
        .map_err(|err| match err {
            NotFound => ApplicationError {
                status: 404,
                message: format!("user {:?} not found", family_id.to_string()),
            },
            _ => ApplicationError {
                status: 500,
                message: format!("internal server error occured"),
            },
        })
}

#[post("")]
async fn create_family(
    db: web::Data<DbPool>,
    data: web::Json<NewFamily>,
) -> Result<HttpResponse, ApplicationError> {
    let mut conn = db
        .get()
        .await
        .expect("Failed to get connection from DB Pool");

    FamilyRepository::create(&mut conn, data.into_inner())
        .await
        .map(|family| {
            HttpResponse::Created().json(json!(
                {
                    "result": "ok",
                    "data": family
                }
            ))
        })
        .map_err(|_| ApplicationError {
            status: 500,
            message: format!("internal server error occured"),
        })
}

#[put("/{family_id}")]
async fn update_family(
    db: web::Data<DbPool>,
    path: web::Path<String>,
    data: web::Json<Family>,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = match Uuid::parse_str(&family_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError {
                message: format!("user {:?} not found", family_id.to_string()),
                status: 404,
            })
        }
    };

    let mut conn = db
        .get()
        .await
        .expect("Failed to get connection from DB Pool");

    FamilyRepository::update(&mut conn, uid, data.into_inner())
        .await
        .map(|family| {
            HttpResponse::Ok().json(json!(
                {
                    "result": "ok",
                    "data": family
                }
            ))
        })
        .map_err(|err| match err {
            NotFound => ApplicationError {
                status: 404,
                message: format!("user {:?} not found", family_id.to_string()),
            },
            _ => ApplicationError {
                status: 500,
                message: format!("internal server error occured"),
            },
        })
}

#[delete("/{family_id}")]
async fn delete_family(
    db: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApplicationError> {
    let family_id = path.into_inner();

    // try parse the family_id if a valid uuid
    // if it not valid, then return 404 not found
    let uid = match Uuid::parse_str(&family_id) {
        Ok(uid) => uid,
        Err(_) => {
            return Err(ApplicationError {
                message: format!("user {:?} not found", family_id.to_string()),
                status: 404,
            })
        }
    };

    let mut conn = db
        .get()
        .await
        .expect("Failed to get connection from DB Pool");

    FamilyRepository::delete(&mut conn, uid)
        .await
        .map(|_| HttpResponse::new(StatusCode::NO_CONTENT))
        .map_err(|err| match err {
            NotFound => ApplicationError {
                status: 404,
                message: format!("user {:?} not found", family_id.to_string()),
            },
            _ => ApplicationError {
                status: 500,
                message: format!("internal server error occured"),
            },
        })
}
