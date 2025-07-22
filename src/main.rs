use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rgamesync_config::{GameSyncConfig, GetConfigurationError};
use rgamesync_rclone::RCloneError;
use thiserror::Error;

use crate::commands::config::run_config;
use crate::commands::sync::{SyncArgs, run_sync};
use crate::commands::version::run_version;
use crate::context::GameSyncContext;



mod commands;
mod context;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a configuration file.
    /// Defaults to `config.toml` in the platform-specific configuration directory for this application.
    #[arg(short, long)]
    config_path: Option<PathBuf>,

    /// Makes all calls to `rclone` interactive. Useful if you want to be careful about what you're doing.
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Makes all calls to `rclone` a dry run. Useful for testing things out before actually overwriting saves.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Version,
    Config,
    Sync(SyncArgs),
}



#[derive(Error, Debug)]
enum ProgramError {
    #[error(transparent)]
    RClone(#[from] RCloneError),
    #[error(transparent)]
    GetConfiguration(#[from] GetConfigurationError),
    #[error("No configuration file found. Run the `rgamesync config` command to make one.")]
    NoConfiguration,
}



fn run_program(args: Args) -> anyhow::Result<()> {
    // These commands don't require a top-level configuration.
    match args.command {
        Command::Version => return Ok(run_version()?),
        Command::Config => return Ok(run_config()?),
        _ => {}
    }

    let config = if let Some(config_path) = args.config_path {
        GameSyncConfig::parse_config_file(config_path)?
    } else {
        GameSyncConfig::get_configuration().map_err(|err| {
            if let GetConfigurationError::NoConfigFile(_path) = err {
                ProgramError::NoConfiguration
            } else {
                err.into()
            }
        })?
    };

    let mut opts: Vec<String> = vec![];
    if args.interactive {
        opts.push("-i".to_owned());
    }
    if args.dry_run {
        opts.push("--dry-run".to_owned());
    }
    let ctx = GameSyncContext::new(config, opts);

    match args.command {
        // Already captured in the above `match` expression.
        Command::Version | Command::Config => unreachable!(),

        Command::Sync(sync_args) => run_sync(&ctx, sync_args)?,
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(err) = run_program(args) {
        eprintln!("{}", err);
    }
}
