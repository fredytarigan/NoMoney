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
