use std::str::FromStr;

/* internal */
use super::types::AppEnv;
use crate::Config;

/* implement ToString for AppEnv enum */
impl ToString for AppEnv {
    fn to_string(&self) -> String {
        match self {
            AppEnv::Development => String::from("development"),
            AppEnv::Production => String::from("production"),
        }
    }
}

/* implement FromStr for AppEnv enum */
impl FromStr for AppEnv {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "production" => Ok(AppEnv::Production),
            _ => Ok(AppEnv::Development),
        }
    }
}

impl Config {
    /*
        Init configuration from app.env file and also
        grab all available environment variables
        configuration from app.env will take precedence.
        Some configuration also have default value.
    */
    pub fn init() -> Self {
        match dotenvy::from_filename("./config/app.env") {
            Ok(path) => {
                println!(
                    "successfully read configuration file from path {}",
                    path.display()
                );
            }
            Err(e) => {
                eprintln!("could not load configuration from file with error {}", e);
                eprintln!("will try to construct configuration from environment variables");
                eprintln!("configuration without default value will result on failed to start the application");
            }
        };

        let (app_name, app_env, app_version) = Config::setup_base_config();
        let (server_addr, server_port, server_timeout) = Config::setup_server_config();

        Self {
            app_name,
            app_env,
            app_version,
            server_addr,
            server_port,
            server_timeout,
        }
    }

    /* setup base configurations */
    fn setup_base_config() -> (String, AppEnv, String) {
        let name = dotenvy::var("APP_NAME")
            .unwrap_or(String::from("NoMoney"))
            .parse::<String>()
            .expect("invalid value for `APP_NAME`, expected `String`");

        let env = match dotenvy::var("APP_ENV") {
            Ok(app) => AppEnv::from_str(&app).unwrap(),
            Err(_) => AppEnv::Development,
        };

        let version = dotenvy::var("APP_VERSION")
            .unwrap_or(String::from("undefined"))
            .parse::<String>()
            .expect("invalid value for `APP_ENV`, expected `String`");

        (name, env, version)
    }

    /* setup server configuration */
    fn setup_server_config() -> (String, u16, u64) {
        let addr = dotenvy::var("SERVER_ADDR")
            .unwrap_or(String::from("127.0.0.1"))
            .parse::<String>()
            .expect("invalid value for `SERVER_ADDR`, expected `String`");

        let port = dotenvy::var("SERVER_PORT")
            .unwrap_or(String::from("8083"))
            .parse::<u16>()
            .expect("invalid value for `SERVER_PORT`, expected `u16`");
        let timeout = dotenvy::var("SERVER_DEFAULT_TIMEOUT")
            .unwrap_or(String::from("30"))
            .parse::<u64>()
            .expect("invalid value for `SERVER_DEFAULT_TIMEOUT`, expected `u16`");

        (addr, port, timeout)
    }
}
