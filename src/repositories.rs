use crate::models::Family;
use crate::schema::*;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct FamilyRepository;

impl FamilyRepository {
    pub async fn find_all(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Family>> {
        families::table.limit(limit).get_results(c).await
    }
}
