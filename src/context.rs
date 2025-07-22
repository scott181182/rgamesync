use std::ffi::OsString;

use rgamesync_config::{GameConfig, GameSyncConfig};
use rgamesync_rclone::RCloneError;



pub struct GameSyncContext {
    pub config: GameSyncConfig,
    opts: Vec<String>,
}

impl GameSyncContext {
    pub fn new(config: GameSyncConfig, opts: Vec<String>) -> Self {
        Self { config, opts }
    }

    pub fn run_rclone(&self, args: &[&str]) -> Result<(), RCloneError> {
        let args = self
            .opts
            .iter()
            .map(OsString::from)
            .chain(args.iter().map(OsString::from))
            .collect::<Vec<_>>();

        #[cfg(debug_assertions)]
        eprintln!("command: {:?}", &args);

        rgamesync_rclone::run_rclone(args)
    }

    pub fn get_remote_path(&self, game: &GameConfig) -> String {
        let remote_root = self.config.remote.path
            .clone()
            .unwrap_or("rgamesync".into());
        let remote_path = remote_root.join("games").join(&game.name).join("sync");

        format!("{}:{}", self.config.remote.name, remote_path.to_string_lossy())
    }
}
