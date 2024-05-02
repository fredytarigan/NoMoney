use sqlx::{migrate::Migrator, postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{error, info};

/* internal dependency */
use super::types::{Database, DbPool};
use crate::common::configs::Config;

impl Database {
    pub async fn new(config: &Config) -> Self {
        Self {
            db_url: config.database.url.to_owned(),
            max_connection: config.database.max_connection.to_owned(),
            acquire_timeout: config.database.acquire_timeout.to_owned(),
            idle_timeout: config.database.idle_timeout.to_owned(),
        }
    }

    pub async fn init(&self) -> DbPool {
        match PgPoolOptions::new()
            .max_connections(self.max_connection)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout))
            .idle_timeout(Duration::from_secs(self.idle_timeout))
            .connect(&self.db_url)
            .await
        {
            Ok(pool) => {
                info!("successfully initialize database connection pool");
                pool
            }
            Err(e) => {
                error!("failed to setup database connection pool with error: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub async fn run_migration(pool: PgPool) {
        static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

        // apply migrations
        match MIGRATOR.run(&pool).await {
            Ok(_) => {
                info!("applying pending migration success")
            }
            Err(e) => {
                error!("unable to apply pending migration with error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
