use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub rust_log: Option<String>,
    pub database_url: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_exp_secs: u64,
}

impl AppConfig {
    pub fn from_files(crate_config: &str) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("../../../config.toml").required(true)) // workspace config
            .add_source(config::File::with_name(crate_config).required(true)) // crate config
            .build()?
            .try_deserialize()
    }

    pub fn is_production(&self) -> bool {
        self.rust_log.as_deref() == Some("info")
    }
}
