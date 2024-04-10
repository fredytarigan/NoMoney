use crate::models::{Family, NewFamily};
use crate::schema::*;
use uuid::Uuid;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct FamilyRepository;

impl FamilyRepository {
    pub async fn find_all(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Family>> {
        families::table.limit(limit).get_results(conn).await
    }

    pub async fn find_by_id(conn: &mut AsyncPgConnection, id: Uuid) -> QueryResult<Family> {
        families::table.find(id).get_result(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, mut data: NewFamily) -> QueryResult<Family> {
        let id = uuid::Uuid::new_v4();

        // inject generated uuid into data
        data.id = id;

        diesel::insert_into(families::table)
            .values(data)
            .get_result(conn)
            .await
    }
}
