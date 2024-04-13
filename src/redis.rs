use mobc::{Connection, Pool};
use mobc_redis::{redis, RedisConnectionManager};
use std::time::Duration;

pub type CachePool = Pool<RedisConnectionManager>;
pub type CacheConn = Connection<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

fn get_redis_config() -> String {
    match dotenvy::var("REDIS_URL") {
        Ok(config) => config,
        Err(_) => {
            // construct redis url from environment variables
            // this is the last resort
            let redis_host = dotenvy::var("REDIS_HOST").unwrap_or(String::from("127.0.0.1"));
            let redis_port = dotenvy::var("REDIS_PORT").unwrap_or(String::from("6379"));

            format!("redis://{}:{}/", redis_host, redis_port,)
        }
    }
}

pub async fn initialize_redis_pool() -> CachePool {
    let redis_config = get_redis_config();

    let client = redis::Client::open(redis_config).unwrap();
    let manager = RedisConnectionManager::new(client);

    Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager)
}
