use super::models::*;
use crate::{app::utils::hash_password, errors::ApplicationError, schema::*};
use chrono::Utc;
use uuid::Uuid;

use diesel::prelude::*;
use diesel::sql_query;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct Repository;

impl Repository {
    pub async fn find_all(
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> Result<Vec<GetUser>, ApplicationError> {
        // let users = users::table.limit(limit).get_results(conn).await?;
        let users = sql_query(format!(
            "SELECT 
                id, 
                username, 
                first_name,
                last_name,
                active,
                family_id,
                role_id,
                email,
                email_validated,
                created_at,
                updated_at 
            FROM 
                users 
            LIMIT {}",
            limit
        ))
        .load::<GetUser>(conn)
        .await?;

        Ok(users)
    }

    pub async fn find_by_id(
        conn: &mut AsyncPgConnection,
        id: Uuid,
    ) -> Result<GetUser, ApplicationError> {
        let users = sql_query(format!(
            "SELECT
                id,
                username,
                first_name,
                last_name,
                active,
                family_id,
                role_id,
                email,
                email_validated,
                created_at,
                updated_at
            FROM
                users
            WHERE
                id = '{}'
            ",
            id
        ))
        .get_result::<GetUser>(conn)
        .await?;

        Ok(users)
    }

    pub async fn _find_by_username(
        conn: &mut AsyncPgConnection,
        username: &String,
    ) -> Result<GetUser, ApplicationError> {
        let users = sql_query(format!(
            "SELECT
                id,
                username,
                first_name,
                last_name,
                active,
                family_id,
                role_id,
                email,
                email_validated,
                created_at,
                updated_at
            FROM
                users
            WHERE
                username = '{}'
            ",
            username
        ))
        .get_result::<GetUser>(conn)
        .await?;

        Ok(users)
    }

    pub async fn login_by_username(
        conn: &mut AsyncPgConnection,
        username: &String,
    ) -> Result<LoginUser, ApplicationError> {
        let users = sql_query(format!(
            "SELECT
                id,
                username,
                password
            FROM
                users
            WHERE
                username = '{}'
            ",
            username,
        ))
        .get_result::<LoginUser>(conn)
        .await?;

        Ok(users)
    }

    pub async fn _find_by_family_id(
        conn: &mut AsyncPgConnection,
        family_id: Uuid,
    ) -> Result<Vec<User>, ApplicationError> {
        let users = users::table
            .filter(users::family_id.eq(family_id))
            .get_results(conn)
            .await?;

        Ok(users)
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        mut data: CreateUser,
    ) -> Result<GetUser, ApplicationError> {
        data.password = hash_password(data.password)?;

        let users = diesel::insert_into(users::table)
            .values(data)
            .get_result::<User>(conn)
            .await?;

        Ok(users.into())
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: Uuid,
        mut data: User,
    ) -> Result<GetUser, ApplicationError> {
        // set updated time into now()
        let updated_time = Utc::now().naive_utc();
        data.updated_at = updated_time;

        let users = diesel::update(users::table.find(id))
            .set((
                users::username.eq(data.username),
                users::first_name.eq(data.first_name),
                users::last_name.eq(data.last_name),
                users::active.eq(data.active),
                users::family_id.eq(data.family_id),
                users::role_id.eq(data.role_id),
                users::email.eq(data.email),
                users::email_validated.eq(data.email_validated),
                users::updated_at.eq(data.updated_at),
            ))
            .get_result::<User>(conn)
            .await?;

        Ok(users.into())
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: Uuid) -> Result<usize, ApplicationError> {
        let result = diesel::delete(users::table.find(id)).execute(conn).await?;

        Ok(result)
    }
}
