use crate::parameters::ParameterSettings;
use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub inventree_url: String,
    pub inventree_token: String,
    pub category_id: u64,
    pub sqlite_db_path: String,
    pub parameters: ParameterSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            inventree_url: "http://localhost:8000".to_string(),
            inventree_token: "0123456789abcdef0123456789abcdef01234567".to_string(),
            category_id: 0,
            sqlite_db_path: "sqlite://proxy_data.db".to_string(),
            parameters: Default::default(),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        if !Path::new("settings.json").exists() {
            serde_json::to_writer_pretty(
                std::fs::File::create("settings.json").unwrap(),
                &Settings::default(),
            )
            .unwrap();
        }

        let builder = Config::builder()
            .add_source(File::from_str(
                &serde_json::to_string(&Settings::default()).unwrap(),
                FileFormat::Json,
            ))
            .add_source(File::with_name("settings.json"));

        let should_be_settings: Settings = builder.build_cloned()?.try_deserialize()?;
        serde_json::to_writer_pretty(
            std::fs::File::create("settings.json").unwrap(),
            &should_be_settings,
        )
        .unwrap();

        let real_settings = builder
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
            .build()?;

        real_settings.try_deserialize()
    }
}
