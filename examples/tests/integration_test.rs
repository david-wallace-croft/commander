use ::assert_cmd::assert::Assert;
use ::assert_cmd::cargo::CargoError;
use ::assert_cmd::prelude::*;
use ::std::process::Command;

#[test]
fn test_output() {
  let result: Result<Command, CargoError> =
    Command::cargo_bin("examples/example_commander");

  let mut command: Command = result.expect("Binary not found");

  let assert: Assert = command.assert();

  let success: Assert = assert.success();

  let _stdout: Assert =
    success.stdout("\nWhat is your name? [World]: Hello, World!\n");
}
