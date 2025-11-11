#[test]
fn parses_basic_args() {
    let args = ["rglt", "needle", "file.txt"];
    let cfg = ripgrep_lite::Config::from_args(args).unwrap();
    assert_eq!(cfg.pattern, "needle");
    assert_eq!(cfg.paths, vec![std::path::PathBuf::from("file.txt")]);
    assert!(!cfg.ignore_case);
}
