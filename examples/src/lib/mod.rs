//==============================================================================
//! CroftSoft Commander library usage example application
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-04-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

pub mod constants;
#[cfg(test)]
mod test;

use commander::parse::CommanderParseError;
use commander::print::print_help;
use commander::print::print_unrecognized_options;
use constants::*;
use std::io::{stdin, stdout, Error, Stdin, Write};

#[derive(Debug)]
pub struct OptionValues {
  pub help_wanted: bool,
  pub interactive: Result<bool, CommanderParseError>,
  pub name_option: Option<String>,
  pub unrecognized: Option<Vec<String>>,
}

pub fn ask(
  prompt: &str,
  default: &str,
) -> String {
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
      },
      Err(error) => println!("ERROR: {}", error),
    }
  }
}

pub fn main(option_values: OptionValues) {
  // println!("{:?}", option_values);
  // println!("{:#?}", option_values);
  if option_values.help_wanted {
    print_help(&HELP_INFO);
    return;
  }
  if option_values.unrecognized.is_some() {
    print_unrecognized_options(&option_values.unrecognized.unwrap());
    return;
  }
  let greeting: String = make_greeting(option_values);
  println!("{}", greeting);
}

// private functions

fn make_greeting(option_values: OptionValues) -> String {
  let name: String = match option_values.name_option {
    Some(arg_name) => {
      if option_values.interactive.is_ok() && option_values.interactive.unwrap()
      {
        ask(NAME_PROMPT, &arg_name)
      } else {
        arg_name
      }
    },
    None => {
      if option_values.interactive.is_ok() && option_values.interactive.unwrap()
      {
        ask(NAME_PROMPT, NAME_DEFAULT)
      } else {
        NAME_DEFAULT.to_string()
      }
    },
  };
  format!("Hello, {}!", name)
}
