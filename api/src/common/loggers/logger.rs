use tracing::{info, Level};
use tracing_subscriber::{fmt::writer::MakeWriterExt, prelude::*, Registry};

/* internal dependency */
use super::types::Logger;
use crate::common::configs::Config;

impl Logger {
    pub fn new(config: &Config) -> Self {
        let level = match config.base.env.to_string().as_str() {
            "production" => Level::INFO,
            _ => Level::DEBUG,
        };

        Self { level }
    }

    pub fn init(&self) -> Result<(), anyhow::Error> {
        info!("Setting logger with level {}", self.level);

        let stdout = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stdout.with_max_level(self.level))
            .json();

        let subscriber = Registry::default().with(stdout);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to set default subscriber");

        Ok(())
    }
}
