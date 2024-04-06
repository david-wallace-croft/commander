use ::assert_cmd::assert::Assert;
use ::assert_cmd::cargo::CargoError;
use ::assert_cmd::prelude::*;
use ::std::process::Command;

#[test]
fn test_output_no_args() {
  let result: Result<Command, CargoError> =
    Command::cargo_bin("examples/example_commander");

  let mut command: Command = result.expect("Binary not found");

  let assert: Assert = command.assert();

  let success: Assert = assert.success();

  let _stdout: Assert =
    success.stdout("\nWhat is your name? [World]: Hello, World!\n");
}

#[test]
fn test_output_args_help() {
  Command::cargo_bin("examples/example_commander")
    .expect("Binary not found")
    .args(&["--help"])
    .assert()
    .success()
    .stdout(
      r#"
CroftSoft Commander Example
Copyright © 2022-2024 CroftSoft Inc
David Wallace Croft <david@CroftSoft.com>
Command-line arguments parser example

OPTIONS:
  -h, --help         Show command-line options
  -i, --interactive  true/false, defaults to true
  -n, --name         Any value not starting with a hyphen (-)
"#,
    );
}

#[test]
fn test_output_args_name() {
  Command::cargo_bin("examples/example_commander")
    .expect("Binary not found")
    .args(&[
      "--name", "David",
    ])
    .assert()
    .success()
    .stdout("\nWhat is your name? [David]: Hello, David!\n");
}
