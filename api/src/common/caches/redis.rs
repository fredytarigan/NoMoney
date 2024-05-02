use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::time::Duration;
use tracing::{error, info};

/* internal dependencies */
use super::types::{Cache, CachePool};
use crate::common::configs::Config;

impl Cache {
    pub async fn new(config: &Config) -> Self {
        Self {
            url: config.cache.url.to_owned(),
            pool_max_open: config.cache.pool_max_open,
            pool_max_idle: config.cache.pool_max_idle,
            pool_timeout_seconds: config.cache.pool_timeout_seconds,
            pool_expire_seconds: config.cache.pool_expire_seconds,
        }
    }

    pub async fn init(&self) -> CachePool {
        let manager = match RedisConnectionManager::new(self.url.to_owned()) {
            Ok(manager) => {
                info!("successfully setup cache connection manager");
                manager
            }
            Err(e) => {
                error!("failed to setup cache connection manager with error: {}", e);
                std::process::exit(1);
            }
        };

        match Pool::builder()
            .connection_timeout(Duration::from_secs(self.pool_timeout_seconds))
            .max_lifetime(Duration::from_secs(self.pool_expire_seconds))
            .build(manager)
            .await
        {
            Ok(pool) => {
                info!("successfully setup cache connection pool");
                pool
            }
            Err(e) => {
                error!("unable to setup cache connection pool with error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
