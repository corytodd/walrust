use walrust::config::Config;

#[test]
fn test_new_config_is_empty() {
    let config = Config::new();
    assert_eq!(config, Config::new());
}
