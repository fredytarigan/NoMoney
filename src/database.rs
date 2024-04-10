use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};

use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn initialize_db_pool() -> DbPool {
    // construct database config from environment variables
    let db_config = match dotenvy::var("DATABASE_URL") {
        Ok(config) => config,
        Err(_) => {
            // construct database url from environment variables
            // this is the last resort
            let db_host = dotenvy::var("DB_HOST").unwrap_or(String::from("127.0.0.1"));
            let db_port = dotenvy::var("DB_PORT").unwrap_or(String::from("5432"));
            let db_user = dotenvy::var("DB_USER").unwrap_or(String::from("postgres"));
            let db_password = dotenvy::var("DB_PASSWORD").unwrap_or(String::from(""));
            let db_name = dotenvy::var("DB_NAME").unwrap_or(String::from("nomoney"));

            format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name,
            )
        }
    };

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_config);

    Pool::builder()
        .build(config)
        .await
        .expect("Unable to connect into Database")
}
