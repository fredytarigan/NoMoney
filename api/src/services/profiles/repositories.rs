use uuid::Uuid;

/* internal dependencies */
use super::models::UserProfile;
use crate::{common::databases::DbPool, utils::errors::ApiError};

pub struct Repository;

impl Repository {
    pub async fn get_profile_by_id(db: &DbPool, user_id: Uuid) -> Result<UserProfile, ApiError> {
        let user = sqlx::query_as!(
            UserProfile,
            "
            SELECT
                u.user_id,
                u.username,
                u.email,
                u.first_name,
                u.last_name,
                u.avatar_path,
                r.role_name,
                f.family_name
            FROM
                users u
            LEFT JOIN
                roles r
            ON
                u.role_id = r.role_id
            LEFT JOIN
                families f
            ON
                u.family_id = f.family_id
            WHERE
                user_id = $1
            ",
            user_id
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }
}
