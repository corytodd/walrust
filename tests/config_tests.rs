use walrust::config::Config;

#[test]
fn test_new_config_is_default() {
    let config = Config::new(None);
    assert_eq!(config, Config::default());
}

#[test]
fn test_new_config_with_custom_depth() {
    let custom_depth = 10;
    let config = Config::new(Some(custom_depth));
    assert_eq!(config.directory_scan_depth, custom_depth);
}
