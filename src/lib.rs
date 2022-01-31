pub mod constants;

use constants::*;
use std::io::{stdin, stdout, Error, Stdin, Write};

#[derive(Debug)]
pub struct MainArgs {
  pub help_wanted: bool,
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

pub fn main(main_args: MainArgs) {
  // println!("{:?}", main_args);
  // println!("{:#?}", main_args);
  if main_args.help_wanted {
    show_help();
    return;
  }
  let greeting: String = make_greeting(main_args);
  println!("{}", greeting);
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

fn show_help() {
  println!("{}", APP_NAME);
  println!("{}", APP_AUTHOR);
  println!("{}", APP_ABOUT);
  println!();
  println!("OPTIONS:");
  println!("  -h, --help");
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_make_greeting_when_name_none() {
    let main_args: MainArgs = MainArgs {
      help_wanted: false,
      interactive: false,
      name_option: None,
    };
    let actual_greeting = make_greeting(main_args);
    assert_eq!(actual_greeting, "Hello, World!");
  }

  #[test]
  fn test_make_greeting_when_name_some() {
    let main_args: MainArgs = MainArgs {
      help_wanted: false,
      interactive: false,
      name_option: Some(String::from("Test")),
    };
    let actual_greeting = make_greeting(main_args);
    assert_eq!(actual_greeting, "Hello, Test!");
  }
}
