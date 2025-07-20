use std::ffi::OsString;

use rgamesync_config::GameSyncConfig;
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
            .chain(args.into_iter().map(OsString::from))
            .collect::<Vec<_>>();
        rgamesync_rclone::run_rclone(args)
    }
}
