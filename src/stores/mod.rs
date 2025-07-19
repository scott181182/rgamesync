use std::ffi::OsString;
use std::fs::read_dir;
use std::path::PathBuf;

use thiserror::Error;



mod game;
pub use game::GameEntry;



#[derive(Error, Debug)]
pub enum ListGameError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not parse game directory name for {0:?}")]
    Os(OsString)
}



pub struct GameStore {
    pub name: String,
    pub path: PathBuf,
}

impl GameStore {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    

    pub fn list_games(&self) -> Result<Vec<GameEntry>, ListGameError> {
        read_dir(&self.path)?
            .filter_map(Result::ok)
            .filter(|ent| ent.path().is_dir())
            .map(|ent| Ok(GameEntry::new(
                ent.file_name().into_string().map_err(ListGameError::Os)?,
                ent.path()
            )))
            .collect::<Result<Vec<_>, ListGameError>>()
    }

    pub fn filter_games(&self, games: &Vec<String>) -> Result<Vec<GameEntry>, ListGameError> {
        let filtered_games = self.list_games()?.into_iter()
            .filter(|game| games.contains(&game.name))
            .collect();
        Ok(filtered_games)
    }
}