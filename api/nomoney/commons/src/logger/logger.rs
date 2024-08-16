use tracing::{info, Level};
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::*, Registry};

/* dependency from internal */
use super::types::Logger;
use crate::{AppEnv, Config};

impl Logger {
    pub fn new(config: &Config) -> Self {
        let level = match config.app_env {
            AppEnv::Production => Level::INFO,
            AppEnv::Development => Level::DEBUG,
        };

        Self { level }
    }

    pub fn init(&self) -> Result<(), std::io::Error> {
        info!("Setting up logger with level {}", self.level);

        let stdout = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stdout.with_max_level(self.level))
            .json();

        let subscriber = Registry::default().with(stdout);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to set default subscriber");

        Ok(())
    }
}
