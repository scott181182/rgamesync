use clap::Args;
use rgamesync_config::GameList;

use crate::{context::GameSyncContext, stores::{GameEntry, GameStore, ListGameError}};

#[derive(Args, Debug)]
pub struct SyncArgs {
    /// Sync specific games. If none are specified, sync all games from the config file.
    games: Vec<String>
}



fn get_games_from_list(games: &Vec<String>, list: &GameList, store: &GameStore) -> Result<Vec<GameEntry>, ListGameError> {
    if !games.is_empty() {
        store.filter_games(games)
    } else {
        match list {
            GameList::All => store.list_games(),
            GameList::List(items) => store.filter_games(items),
        }
    }
    
}

pub fn run_sync(
    ctx: &GameSyncContext,
    args: SyncArgs
) -> anyhow::Result<()> {
    let stores = ctx.config.stores.iter()
        .map(|(name, path)| GameStore::new(name.clone(), path.clone()))
        .collect::<Vec<GameStore>>();
    let store = &stores[0];

    let games = get_games_from_list(&args.games, &ctx.config.games, store)?;
    for game in games {
        // TODO: sync each game.
        ctx.run_rclone(&["sync"])?;
    }

    Ok(())
}