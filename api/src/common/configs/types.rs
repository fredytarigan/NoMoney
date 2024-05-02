/*
    AppEnv enum
    defined in which environment the app is running
*/
#[derive(Debug, Clone)]
pub enum AppEnv {
    Development,
    Production,
}

/*
    Base struct for Config
*/
#[derive(Debug, Clone)]
pub struct Config {
    pub base: BaseConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
}

/*
    Base configuration
*/
#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub env: AppEnv,
    pub version: String,
}

/*
    Server configuration
*/
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

/*
    Database configuration
*/
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connection: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
}

/*
    Caching configuration
*/
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub url: String,
    pub pool_max_open: u64,
    pub pool_max_idle: u64,
    pub pool_timeout_seconds: u64,
    pub pool_expire_seconds: u64,
}
