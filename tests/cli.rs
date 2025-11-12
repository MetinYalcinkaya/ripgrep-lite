use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn no_args() {
    let mut cmd = cargo_bin_cmd!("rglt");
    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("Usage"));
}
