#[test]
fn parses_basic_args() {
    let args = ["rglt", "black", "tests/data/poem.txt"];
    let cfg = ripgrep_lite::Config::from_args(args).unwrap();
    assert_eq!(cfg.pattern, "black");
    assert_eq!(
        cfg.paths,
        vec![std::path::PathBuf::from("tests/data/poem.txt")]
    );
    assert!(!cfg.ignore_case);
}

#[test]
fn errors_when_path_missing() {
    let args = ["rglt", "black"];
    let err = ripgrep_lite::Config::from_args(args).unwrap_err();
    assert_eq!(err, ripgrep_lite::ConfigError::MissingPath);
}

#[test]
fn errors_when_pattern_missing() {
    let args = ["rglt"];
    let err = ripgrep_lite::Config::from_args(args).unwrap_err();
    assert_eq!(err, ripgrep_lite::ConfigError::MissingPattern);
}
