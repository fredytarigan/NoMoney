use std::str::FromStr;

use super::{
    types::{AppEnv, BaseConfig, ServerConfig},
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
            .expect("invalid value for ENV, expected String got incompatible value");

        BaseConfig { env, version }
    }

    fn setup_server_config() -> ServerConfig {
        let addr = dotenvy::var("ADDR")
            .unwrap_or(String::from("127.0.0.1"))
            .parse::<String>()
            .expect("invalid value for ADDR, expected String got incompatible value");
        let port = dotenvy::var("PORT")
            .unwrap_or(String::from("8083"))
            .parse::<u16>()
            .expect("invalid value for PORT, expected u16 got incompatible value");

        ServerConfig { addr, port }
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
        }
    }
}
