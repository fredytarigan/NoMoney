pub type DbPool = sqlx::Pool<sqlx::Postgres>;

#[derive(Debug)]
pub struct Database {
    pub db_url: String,
    pub max_connection: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
}
