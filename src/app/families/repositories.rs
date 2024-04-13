use super::models::*;
use crate::{errors::ApplicationError, schema::*};
use chrono::Utc;
use uuid::Uuid;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct Repository;

impl Repository {
    pub async fn find_all(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> Result<Vec<Family>, ApplicationError> {
        let families = families::table.limit(limit).get_results(conn).await?;

        Ok(families)
    }

    pub async fn find_by_id(
        conn: &mut AsyncPgConnection,
        id: Uuid,
    ) -> Result<Family, ApplicationError> {
        let families = families::table.find(id).get_result(conn).await?;

        Ok(families)
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        data: CreateFamily,
    ) -> Result<Family, ApplicationError> {
        let families = diesel::insert_into(families::table)
            .values(data)
            .get_result(conn)
            .await?;

        Ok(families)
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: Uuid,
        mut data: Family,
    ) -> Result<Family, ApplicationError> {
        // set updated time into now()
        let updated_time = Utc::now().naive_utc();
        data.updated_at = updated_time;

        let families = diesel::update(families::table.find(id))
            .set((
                families::name.eq(data.name),
                families::description.eq(data.description),
                families::updated_at.eq(data.updated_at),
            ))
            .get_result(conn)
            .await?;

        Ok(families)
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: Uuid) -> Result<usize, ApplicationError> {
        let result = diesel::delete(families::table.find(id))
            .execute(conn)
            .await?;

        Ok(result)
    }
}
