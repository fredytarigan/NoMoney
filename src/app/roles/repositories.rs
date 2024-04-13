use super::models::*;
use crate::{errors::ApplicationError, schema::*};
use uuid::Uuid;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct Repository;

impl Repository {
    pub async fn find_all(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> Result<Vec<Role>, ApplicationError> {
        let roles = roles::table.limit(limit).get_results(conn).await?;

        Ok(roles)
    }

    pub async fn find_by_id(
        conn: &mut AsyncPgConnection,
        id: Uuid,
    ) -> Result<Role, ApplicationError> {
        let roles = roles::table.find(id).get_result(conn).await?;

        Ok(roles)
    }

    pub async fn find_by_name(
        conn: &mut AsyncPgConnection,
        name: &String,
    ) -> Result<Role, ApplicationError> {
        let roles = roles::table
            .filter(roles::name.eq(name))
            .get_result(conn)
            .await?;

        Ok(roles)
    }
}
