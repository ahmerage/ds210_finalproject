use std::process::Command;
use assert_cmd::Command as AssertCommand;

#[test]
fn test_integration() {
    let mut cmd = Command::cargo_bin("global_socioeconomic_analysis").unwrap();
    let assert_cmd = AssertCommand::from(cmd);
    let assert_cmd = assert_cmd.assert().success();


    assert_cmd.finish();
}