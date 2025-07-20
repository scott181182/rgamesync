use glob::Pattern;
use rgamesync_config::GameSyncConfig;



macro_rules! test_config {
    ($path:literal) => {{
        const CONFIG_PATH: &str = $path;
        GameSyncConfig::parse_config_file(CONFIG_PATH).expect("Should parse config")
    }};
}


#[test]
fn parse_simple_config() {
    let expected_pattern = Pattern::new("*.sl2").unwrap();

    let mut config = test_config!("tests/data/simple.toml");

    let ds3 = config.games.get_mut(0).expect("Should contain GameConfig for DS3");
    assert_eq!(ds3.save_glob.take().unwrap(), expected_pattern);
}

#[test]
fn parse_no_save_glob_config() {
    let mut config = test_config!("tests/data/no_save_glob.toml");

    let ds3 = config.games.get_mut(0).expect("Should contain GameConfig for DS3");
    assert!(ds3.save_glob.is_none());
}
