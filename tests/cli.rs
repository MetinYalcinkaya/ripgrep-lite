use assert_cmd::cargo::cargo_bin_cmd;

#[test]
fn no_args() {
    let mut cmd = cargo_bin_cmd!("rglt");
    let assert = cmd.arg("").assert();
    assert.failure().code(2);
}
