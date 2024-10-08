//==============================================================================
//! Integration tests that check the standard output.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-04-06
//! - Updated: 2024-08-15
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use ::std::process::Command;

use ::assert_cmd::assert::Assert;
use ::assert_cmd::cargo::CargoError;
use ::assert_cmd::prelude::*;
use ::predicates::prelude::*;

fn make_command() -> Command {
  let result: Result<Command, CargoError> =
    Command::cargo_bin("examples/example_commander");

  result.expect("Binary not found")
}

#[test]
fn test_output_no_args() {
  let mut command: Command = make_command();

  let assert: Assert = command.assert();

  let success: Assert = assert.success();

  let _stdout: Assert =
    success.stdout("\nWhat is your name? [World]: Hello, World!\n");
}

#[test]
fn test_output_args_help() {
  make_command().args(["--help"]).assert().success().stdout(
    r#"
CroftSoft Commander Example
Copyright © 2022-2024 CroftSoft Inc
David Wallace Croft <david@CroftSoft.com>
Command-line arguments parser example

OPTIONS:
  -h, --help         Show command-line options
  -i, --interactive  true/false, defaults to true
  -n, --name         Any value not starting with a hyphen (-)
  -q, --quiet        Suppress the exclamation mark
"#,
  );
}

#[test]
fn test_output_args_name_long() {
  make_command()
    .args(["--name=David"])
    .assert()
    .success()
    .stdout("\nWhat is your name? [David]: Hello, David!\n");
}

#[test]
fn test_output_args_name_short() {
  make_command()
    .args(["-n=David"])
    .assert()
    .success()
    .stdout("\nWhat is your name? [David]: Hello, David!\n");
}

#[test]
fn test_output_args_non_interactive_equals() {
  make_command()
    .args(["-i=false"])
    .assert()
    .success()
    .stdout("Hello, World!\n");
}

#[test]
fn test_output_args_non_interactive_quiet() {
  make_command()
    .args(["-qi=false"])
    .assert()
    .success()
    .stdout("Hello, World.\n");
}

#[test]
fn test_output_args_non_interactive_long_equals() {
  make_command()
    .args(["--interactive=false"])
    .assert()
    .success()
    .stdout("Hello, World!\n");
}

#[test]
fn test_output_args_non_interactive_name() {
  make_command()
    .args([
      "-i=false", "-n=David",
    ])
    .assert()
    .success()
    .stdout("Hello, David!\n");
}

#[test]
fn test_output_args_non_interactive_name_predicate() {
  make_command()
    .args([
      "--interactive=false",
      "--name=David",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("David"));
}

#[test]
fn test_output_args_unknown_long() {
  make_command()
    .args(["--unknown"])
    .assert()
    // TODO: Should this be failure?
    .success()
    // TODO: Should this go to standard error?
    .stdout("Unknown option at argument index 1: \"unknown\"\n");
}

#[test]
fn test_output_args_unknown_short() {
  make_command()
    .args(["-u"])
    .assert()
    // TODO: Should this be failure?
    .success()
    // TODO: Should this go to standard error?
    .stdout("Unknown option at argument index 1 character index 0: 'u'\n");
}

#[test]
fn test_output_args_unknown_option_value() {
  make_command()
    .args(["--unknown=true"])
    .assert()
    // TODO: Should this be failure?
    .success()
    // TODO: Should this go to standard error?
    .stdout("Unknown option at argument index 1: \"unknown\"\n");
}
