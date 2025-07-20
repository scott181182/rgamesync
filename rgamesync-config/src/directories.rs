use std::fs::create_dir_all;
use std::path::PathBuf;

use directories::ProjectDirs;
use thiserror::Error;



#[derive(Error, Debug)]
pub enum ConfigDirectoryError {
    #[error("Couldn't determine config directory for application 'rgamesync'")]
    NoConfigDir,
    #[error("Couldn't make config directory for application 'rgamesync' at '{0}'")]
    CouldNotMakeConfigDir(PathBuf),
}

pub fn get_config_path() -> Result<PathBuf, ConfigDirectoryError> {
    let dirs = ProjectDirs::from("com", "rgamesync", "rgamesync").ok_or(ConfigDirectoryError::NoConfigDir)?;

    let config_dir = dirs.config_dir();
    create_dir_all(config_dir).map_err(|_err| ConfigDirectoryError::CouldNotMakeConfigDir(config_dir.to_owned()))?;

    let config_file_path = dirs.config_dir().join("config.toml");
    Ok(config_file_path)
}
