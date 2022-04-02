//==============================================================================
//! CroftSoft Commander library usage example application
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-01-15
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

mod constants;

use commander::parse::{
  parse_option_type_bool_with_optional_value,
  parse_option_type_bool_without_value,
  parse_option_type_string_with_required_value,
};
use commander::print::print_help;
use constants::*;
use std::env;
use std::io::{stdin, stdout, Error, Stdin, Write};

#[derive(Debug)]
pub struct MainOptions {
  pub help_wanted: bool,
  pub interactive: bool,
  pub name_option: Option<String>,
}

pub fn ask(prompt: &str, default: &str) -> String {
  loop {
    println!();
    print!("{} [{}]: ", prompt, default);
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

pub fn main(main_options: MainOptions) {
  // println!("{:?}", main_options);
  // println!("{:#?}", main_options);
  if main_options.help_wanted {
    show_help();
    return;
  }
  let greeting: String = make_greeting(main_options);
  println!("{}", greeting);
}

// private functions

fn make_greeting(main_options: MainOptions) -> String {
  let name: String = match main_options.name_option {
    Some(arg_name) => {
      if main_options.interactive {
        ask(NAME_PROMPT, &arg_name)
      } else {
        arg_name
      }
    }
    None => {
      if main_options.interactive {
        ask(NAME_PROMPT, NAME_DEFAULT)
      } else {
        NAME_DEFAULT.to_string()
      }
    }
  };
  format!("Hello, {}!", name)
}

//------------------------------------------------------------------------------
/// Uses the CroftSoft Commander library to parse the application options
//------------------------------------------------------------------------------
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
pub fn make_main_options() -> MainOptions {
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);
  // println!("Args length = {}", length);
  let args_slice: &[String] = &args[1..];
  let help_wanted: bool =
    parse_option_type_bool_without_value(args_slice, &ARG_OPTION_H);
  let interactive: bool =
    parse_option_type_bool_with_optional_value(args_slice, &ARG_OPTION_I);
  // TODO: parse_option_type_string_with_default_value
  let name_option =
    parse_option_type_string_with_required_value(args_slice, &ARG_OPTION_N);
  MainOptions {
    help_wanted,
    interactive,
    name_option,
  }
}

fn show_help() {
  print_help(&HELP_INFO);
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_make_greeting_when_name_none() {
    let main_options: MainOptions = MainOptions {
      help_wanted: false,
      interactive: false,
      name_option: None,
    };
    let actual_greeting = make_greeting(main_options);
    assert_eq!(actual_greeting, "Hello, World!");
  }

  #[test]
  fn test_make_greeting_when_name_some() {
    let main_options: MainOptions = MainOptions {
      help_wanted: false,
      interactive: false,
      name_option: Some(String::from("Test")),
    };
    let actual_greeting = make_greeting(main_options);
    assert_eq!(actual_greeting, "Hello, Test!");
  }
}