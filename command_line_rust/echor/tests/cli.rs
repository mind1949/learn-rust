use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut cmd =  Command::cargo_bin("echor").unwrap();
    cmd.assert()
    .failure()
    .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello")
    .assert()
    .success();
}

