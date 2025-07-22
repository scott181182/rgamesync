use std::mem::swap;

use anyhow::anyhow;
use clap::Args;
use rgamesync_config::GameConfig;

use crate::context::GameSyncContext;



#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct SyncDirectionArg {
    /// Sync to the remote.
    /// Mutually exclusive with --from.
    #[arg(long)]
    to: bool,
    /// Sync from the remote.
    /// Mutually exclusive with --to.
    #[arg(long)]
    from: bool,
}

#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Sync specific games. If none are specified, sync all games from the config file.
    games: Vec<String>,

    #[command(flatten)]
    direction: SyncDirectionArg,
}



fn get_games_from_list(games: &[String], list: &[GameConfig]) -> anyhow::Result<Vec<GameConfig>> {
    if games.is_empty() {
        Ok(list.to_vec())
    } else {
        let filtered_games = list
            .iter()
            .filter(|g| games.contains(&g.name))
            .map(GameConfig::clone)
            .collect();

        Ok(filtered_games)
    }
}

pub fn run_sync(ctx: &GameSyncContext, args: SyncArgs) -> anyhow::Result<()> {
    let games = get_games_from_list(&args.games, &ctx.config.games)?;

    for game in games {
        let mut src = game.save_dir.to_str().ok_or(anyhow!("Could not get valid string from source game path"))?.to_owned();
        let mut dest = ctx.get_remote_path(&game);

        if args.direction.from {
            // Swap src and dest if we want to sync _from_ the remote instead of _to_.
            swap(&mut src, &mut dest);
        }

        let mut args = vec!["sync", &src, &dest];
        if let Some(ref pattern) = game.save_glob {
            args.extend_from_slice(&[ "--include", pattern.as_str() ]);
        }

        ctx.run_rclone(&args)?;
    }

    Ok(())
}
