use std::path::PathBuf;



pub struct GameEntry {
    pub name: String,
    pub path: PathBuf,
}

impl GameEntry {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}