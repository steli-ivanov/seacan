use std::sync::OnceLock;

use crate::{constants, outputln};

#[derive(Debug, serde::Deserialize)]
pub struct Object {
    pub log_days_retention: usize,
    pub number_of_workers: usize,
}

static OBJECT: OnceLock<Object> = OnceLock::new();

/// # Panics
/// if is used without calling `load`
pub fn object<'a>() -> &'a Object {
    OBJECT.get().expect("config was not initialized")
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Failed to create the config file, {0}")]
    CreationFailed(String),
    #[error("Failed to read from the config file, {0}")]
    FailedToReadConfig(String),
    #[error("Invalid config file, {0}")]
    InvalidYamlFile(String),
}

/// # Errors
/// `LoadError` enum
/// # Panics
/// if `load` is called more than once
pub fn load() -> anyhow::Result<()> {
    if !std::fs::exists(constants::CONFIG_DIRECTORY).unwrap_or(false) {
        std::fs::create_dir(constants::CONFIG_DIRECTORY)
            .map_err(|e| LoadError::CreationFailed(e.to_string()))?;
    }

    if !std::fs::exists(constants::CONFIG_FILENAME).unwrap_or(false) {
        std::fs::write(
            constants::CONFIG_FILENAME,
            constants::CONFIG_DEFAULT_SETTINGS,
        )
        .map_err(|e| LoadError::CreationFailed(e.to_string()))?;

        outputln!("default configuration was saved");
    }

    let content = std::fs::read_to_string(constants::CONFIG_FILENAME)
        .map_err(|e| LoadError::FailedToReadConfig(e.to_string()))?;

    let object: Object =
        serde_yaml::from_str(&content).map_err(|e| LoadError::InvalidYamlFile(e.to_string()))?;

    OBJECT.set(object).expect("config was already initialized");

    outputln!("configuration was loaded");

    Ok(())
}
