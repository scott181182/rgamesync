use std::collections::HashMap;
use std::fs::{exists, read_to_string, write};
use std::path::{Path, PathBuf};

use glob::Pattern;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::directories::{ConfigDirectoryError, get_config_path};



//////////////////////////////////
//   Configuration Structures   //
//////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// The name of the game, especially the folder to find it in.
    pub name: String,
    /// The directory to find game saves.
    pub save_dir: PathBuf,
    /// A glob pattern to find save files by. If omitted, will sync all files in the `save_dir`.
    #[serde(default, with = "crate::serde::save_glob")]
    pub save_glob: Option<Pattern>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// The name of the `rclone` remote that should be sync'd.
    pub name: String,
    /// The path within the remote to sync to.
    /// Defaults to `/rgamesync`.
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSyncConfig {
    /// A map from store name to the path of that store's game directory.
    ///
    /// ```toml
    /// [stores]
    /// steam = "/home/user/.var/app/com.valvesoftware.Steam/data/Steam/steamapps/common"
    /// ```
    pub stores: HashMap<String, PathBuf>,
    /// Configuration for games to sync.
    pub games: Vec<GameConfig>,
    /// Configuration for `rclone` remote.
    ///
    /// TODO: added support for multiple remotes.
    pub remote: RemoteConfig,
}



////////////////
//   Errors   //
////////////////

#[derive(Error, Debug)]
pub enum SaveConfigurationError {
    #[error(transparent)]
    Directory(#[from] ConfigDirectoryError),
    #[error(transparent)]
    CouldNotSerialize(#[from] toml::ser::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum GetConfigurationError {
    #[error(transparent)]
    Directory(#[from] ConfigDirectoryError),
    #[error("Couldn't find config file at '{0}'")]
    NoConfigFile(PathBuf),
    #[error("Couldn't read config file at '{0}'")]
    CouldNotReadConfigFile(PathBuf),
    #[error("Couldn't parse configuration: {0}")]
    CouldNotParseConfig(#[from] toml::de::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}



////////////////////////
//   Implementation   //
////////////////////////

impl GameSyncConfig {
    ///
    /// Get a `GameSyncConfig` from the default configuration file location.
    ///
    pub fn get_configuration() -> Result<GameSyncConfig, GetConfigurationError> {
        let config_file_path = get_config_path()?;
        if !exists(&config_file_path)? {
            return Err(GetConfigurationError::NoConfigFile(config_file_path.to_owned()));
        }

        Self::parse_config_file(config_file_path)
    }
    pub fn parse_config_file<P: AsRef<Path>>(path: P) -> Result<GameSyncConfig, GetConfigurationError> {
        let config_data = read_to_string(path.as_ref())
            .map_err(|_err| GetConfigurationError::CouldNotReadConfigFile(path.as_ref().to_owned()))?;

        let config: GameSyncConfig = toml::from_str(&config_data)?;
        Ok(config)
    }

    ///
    /// Save this `GameSyncConfig` to the default configuration file location.
    ///
    pub fn save(&self) -> Result<(), SaveConfigurationError> {
        let config_file_path = get_config_path()?;
        self.save_to(config_file_path)
    }
    ///
    /// Save this `GameSyncConfig` to a specific path.
    ///
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveConfigurationError> {
        let config_data = toml::to_string(self)?;
        write(path, config_data)?;
        Ok(())
    }
}
