#[derive(Debug)]
pub struct Config {
    // base app configuration
    pub app_name: String,
    pub app_env: AppEnv,
    pub app_version: String,

    // api server configuration
    pub server_addr: String,
    pub server_port: u16,
    pub server_timeout: u64,
}

#[derive(Debug, Clone)]
pub enum AppEnv {
    Development,
    Production,
}
