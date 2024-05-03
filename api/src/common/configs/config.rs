use std::str::FromStr;

use super::{
    types::{AppEnv, BaseConfig, CacheConfig, DatabaseConfig, JwtConfig, ServerConfig},
    Config,
};

impl ToString for AppEnv {
    fn to_string(&self) -> String {
        match self {
            AppEnv::Development => String::from("development"),
            AppEnv::Production => String::from("production"),
        }
    }
}

impl FromStr for AppEnv {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "production" => Ok(AppEnv::Production),
            _ => Ok(AppEnv::Development),
        }
    }
}

/*
    Config struct implementation
*/
impl Config {
    fn setup_base_config() -> BaseConfig {
        let env = match dotenvy::var("ENV") {
            Ok(app) => AppEnv::from_str(&app).unwrap(),
            Err(_) => AppEnv::Development,
        };

        let version = dotenvy::var("VERSION")
            .unwrap_or(String::from("undefined"))
            .parse::<String>()
            .expect("invalid value for ENV, expected `String` got incompatible value");

        BaseConfig { env, version }
    }

    fn setup_server_config() -> ServerConfig {
        let addr = dotenvy::var("ADDR")
            .unwrap_or(String::from("127.0.0.1"))
            .parse::<String>()
            .expect("invalid value for ADDR, expected `String` got incompatible value");
        let port = dotenvy::var("PORT")
            .unwrap_or(String::from("8083"))
            .parse::<u16>()
            .expect("invalid value for PORT, expected `u16` got incompatible value");

        ServerConfig { addr, port }
    }

    pub fn setup_database_config() -> DatabaseConfig {
        let db_host = dotenvy::var("DB_HOST")
            .unwrap_or(String::from("127.0.0.1"))
            .parse::<String>()
            .expect("invalid value for DB_HOST, expected `String` got incompatible value");

        let db_port = dotenvy::var("DB_PORT")
            .unwrap_or(String::from("5432"))
            .parse::<String>()
            .expect("invalid value for DB_PORT, expected `String` got incompatible value");

        let db_user = dotenvy::var("DB_USER")
            .unwrap_or(String::from("postgres"))
            .parse::<String>()
            .expect("invalid value for DB_USER, expected `String` got incompatible value");

        let db_password = dotenvy::var("DB_PASSWORD")
            .unwrap_or(String::from("postgres"))
            .parse::<String>()
            .expect("invalid value for DB_PASSWORD, expected `String` got incompatible value");

        let db_name = dotenvy::var("DB_NAME")
            .unwrap_or(String::from("octopus"))
            .parse::<String>()
            .expect("invalid value for DB_NAME, expected `String` got incompatible value");

        let max_connection = dotenvy::var("DB_MAX_CONNECTION")
            .unwrap_or(String::from("10"))
            .parse::<u32>()
            .expect("invalid value for DB_MAX_CONNECTION, expected `u32` got incompatible value");

        let acquire_timeout = dotenvy::var("DB_ACQUIRE_TIMEOUT")
            .unwrap_or(String::from("15"))
            .parse::<u64>()
            .expect("invalid value for DB_ACQUIRE_TIMEOUT, expected `u64` got incompatible value");

        let idle_timeout = dotenvy::var("DB_IDLE_TIMEOUT")
            .unwrap_or(String::from("60"))
            .parse::<u64>()
            .expect("invalid value for DB_IDLE_TIMEOUT expected `u64` got incompatible value");

        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name,
        );

        DatabaseConfig {
            url,
            max_connection,
            acquire_timeout,
            idle_timeout,
        }
    }

    pub fn setup_cache_config() -> CacheConfig {
        let cache_host = dotenvy::var("CACHE_HOST")
            .unwrap_or(String::from("127.0.0.1"))
            .parse::<String>()
            .expect("invalid value for CACHE_HOST, expected `String` got incompatible value");

        let cache_port = dotenvy::var("CACHE_PORT")
            .unwrap_or(String::from("6379"))
            .parse::<String>()
            .expect("invalid value for CACHE_HOST, expected `String` got incompatible value");

        let url = format!("redis://{}:{}", cache_host, cache_port);

        /* cache pool configuration */
        let pool_max_open = dotenvy::var("CACHE_POOL_MAX_OPEN")
            .unwrap_or(String::from("16"))
            .parse::<u64>()
            .expect("invalid value for CACHE_POOL_MAX_OPEN, expected `u64` got incompatible value");

        let pool_max_idle = dotenvy::var("CACHE_POOL_MAX_IDLE")
            .unwrap_or(String::from("8"))
            .parse::<u64>()
            .expect("invalid value for CACHE_POOL_MAX_IDLE, expected `u64` got incompatible value");

        let pool_timeout_seconds = dotenvy::var("CACHE_POOL_TIMEOUT_SECONDS")
            .unwrap_or(String::from("5"))
            .parse::<u64>()
            .expect(
                "invalid value for CACHE_POOL_TIMEOUT_SECONDS, expected `u64` got incompatible value",
            );

        let pool_expire_seconds = dotenvy::var("CACHE_POOL_EXPIRE_SECONDS")
            .unwrap_or(String::from("60"))
            .parse::<u64>()
            .expect(
                "invalid value for CACHE_POOL_EXPIRE_SECONDS, expected `u64` got incompatible value",
            );

        CacheConfig {
            url,
            pool_max_open,
            pool_max_idle,
            pool_timeout_seconds,
            pool_expire_seconds,
        }
    }

    pub fn setup_jwt_config() -> JwtConfig {
        let secret = dotenvy::var("JWT_SECRET")
            .unwrap_or(String::from("S0m3R4nd0MS3cr3T#!"))
            .parse::<String>()
            .expect("invalid value for JWT_SECRET, expected `String` got incompatible value");

        let duration = dotenvy::var("JWT_DURATION")
            .unwrap_or(String::from("28800"))
            .parse::<u64>()
            .expect("invalid value for JWT_DURATION, expected `u64` got incompatible value");

        JwtConfig { secret, duration }
    }
}

/*
    Default implementation for config struct
*/
impl Default for Config {
    fn default() -> Self {
        match dotenvy::from_filename("./config/app.env") {
            Ok(path) => {
                eprintln!(
                    "successfully read configuration file from {}",
                    path.display()
                )
            }
            Err(e) => {
                eprintln!("could not load configuration from file {}", e)
            }
        }

        Self {
            base: Config::setup_base_config(),
            server: Config::setup_server_config(),
            database: Config::setup_database_config(),
            cache: Config::setup_cache_config(),
            jwt: Config::setup_jwt_config(),
        }
    }
}
