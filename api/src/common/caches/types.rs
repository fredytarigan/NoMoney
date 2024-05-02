pub type CachePool = bb8::Pool<bb8_redis::RedisConnectionManager>;

pub struct Cache {
    pub url: String,
    pub pool_max_open: u64,
    pub pool_max_idle: u64,
    pub pool_timeout_seconds: u64,
    pub pool_expire_seconds: u64,
}
