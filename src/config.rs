use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::ConnectionString;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_non_empty_vec")]
    pub connection_strings: Vec<ConnectionString>,
}

fn deserialize_non_empty_vec<'de, D>(deserializer: D) -> Result<Vec<ConnectionString>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<ConnectionString>::deserialize(deserializer)?;
    if vec.is_empty() {
        return Err(serde::de::Error::custom(
            "connection_strings cannot be empty",
        ));
    }
    Ok(vec)
}

impl Config {
    pub async fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read config file '{}'", path.display()))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file '{}'", path.display()))?;

        Ok(config)
    }

    pub async fn generate_to_file(path: &Path) -> Result<()> {
        let example_config = Config {
            connection_strings: vec![
                ConnectionString {
                    name: "example_db".to_string(),
                    uri: "postgresql://user:password@localhost:5432/database".to_string(),
                },
            ],
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.with_context(|| {
                format!("Failed to create directory '{}'", parent.display())
            })?;
        }

        let json = serde_json::to_string_pretty(&example_config)
            .context("Failed to serialize config")?;

        fs::write(path, json)
            .await
            .with_context(|| format!("Failed to write config file '{}'", path.display()))?;

        Ok(())
    }
}

pub fn default_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;

    let app_name = env!("CARGO_PKG_NAME");
    Ok(home.join(format!(".{}", app_name)).join("config.json"))
}
