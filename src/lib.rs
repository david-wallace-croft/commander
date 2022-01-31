use std::io::{stdin, stdout, Error, Stdin, Write};
// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};

pub const APP_NAME: &str = "CroftSoft Commander © 2022 CroftSoft Inc";
pub const APP_ABOUT: &str = "Command-line Rust example";
pub const APP_AUTHOR: &str = "David Wallace Croft, david@croftsoft.com";
pub const ARG_INTERACTIVE_HELP: &str = "prompt user for inputs";
pub const ARG_INTERACTIVE_NAME: &str = "interactive";
pub const ARG_INTERACTIVE_SHORT: char = 'i';
pub const ARG_INTERACTIVE_TAKES_VALUE: bool = true;
pub const ARG_NAME_HELP: &str = "name to greet";
pub const ARG_NAME_NAME: &str = "name";
pub const ARG_NAME_SHORT: char = 'n';
pub const ARG_NAME_TAKES_VALUE: bool = true;
pub const NAME_DEFAULT: &str = "World";
pub const NAME_PROMPT: &str = "What is your name? [World]: ";

#[derive(Debug)]
pub struct MainArgs {
  pub interactive: bool,
  pub name_option: Option<String>,
}

pub fn ask(prompt: &str, default: &str) -> String {
  loop {
    println!();
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut buffer: String = String::new();
    let stdin: Stdin = stdin();
    let result: Result<usize, Error> = stdin.read_line(&mut buffer);
    match result {
      Ok(_) => {
        let trimmed_buffer: &str = buffer.trim();
        if trimmed_buffer.is_empty() {
          return default.to_string();
        }
        return trimmed_buffer.to_string();
      }
      Err(error) => println!("ERROR: {}", error),
    }
  }
}

pub fn main_with_args(main_args: MainArgs) {
  // println!("{:?}", main_args);
  // println!("{:#?}", main_args);
  let greeting: String = make_greeting(main_args);
  println!("{}", greeting);
}

pub fn main_with_clap() {
  let interactive_arg = Arg::new(ARG_INTERACTIVE_NAME)
    .help(ARG_INTERACTIVE_HELP)
    .long(ARG_INTERACTIVE_NAME)
    .short(ARG_INTERACTIVE_SHORT)
    .takes_value(ARG_INTERACTIVE_TAKES_VALUE);
  let name_arg = Arg::new(ARG_NAME_NAME)
    .help(ARG_NAME_HELP)
    .long(ARG_NAME_NAME)
    .short(ARG_NAME_SHORT)
    .takes_value(ARG_NAME_TAKES_VALUE);
  let app: App = App::new(APP_NAME)
    .about(APP_ABOUT)
    .arg(interactive_arg)
    .arg(name_arg)
    .author(APP_AUTHOR);
  let arg_matches: ArgMatches = app.get_matches();
  let arg_match_interactive: Option<&str> =
    arg_matches.value_of(ARG_INTERACTIVE_NAME);
  let arg_match_name: Option<&str> = arg_matches.value_of(ARG_NAME_NAME);
  let interactive: bool = !matches!(arg_match_interactive, Some("false"));
  let name_option: Option<String> = arg_match_name.map(|name| name.to_string());
  let main_args: MainArgs = MainArgs {
    interactive,
    name_option,
  };
  main_with_args(main_args);
}

// private functions

fn make_greeting(main_args: MainArgs) -> String {
  let name: String = match main_args.name_option {
    Some(arg_name) => arg_name,
    None => {
      if main_args.interactive {
        ask(NAME_PROMPT, NAME_DEFAULT)
      } else {
        NAME_DEFAULT.to_string()
      }
    }
  };
  format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_make_greeting_when_name_none() {
    let main_args: MainArgs = MainArgs {
      interactive: false,
      name_option: None,
    };
    let actual_greeting = make_greeting(main_args);
    assert_eq!(actual_greeting, "Hello, World!");
  }

  #[test]
  fn test_make_greeting_when_name_some() {
    let main_args: MainArgs = MainArgs {
      interactive: false,
      name_option: Some(String::from("Test")),
    };
    let actual_greeting = make_greeting(main_args);
    assert_eq!(actual_greeting, "Hello, Test!");
  }
}
