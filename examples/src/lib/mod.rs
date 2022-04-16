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

pub mod constants;
#[cfg(test)]
mod test;

use commander::print::print_help;
use constants::*;
use std::io::{stdin, stdout, Error, Stdin, Write};

#[derive(Debug)]
pub struct OptionValues {
  pub help_wanted: bool,
  pub interactive: bool,
  pub name_option: Option<String>,
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
    show_help();
    return;
  }
  let greeting: String = make_greeting(option_values);
  println!("{}", greeting);
}

// private functions

fn make_greeting(option_values: OptionValues) -> String {
  let name: String = match option_values.name_option {
    Some(arg_name) => {
      if option_values.interactive {
        ask(NAME_PROMPT, &arg_name)
      } else {
        arg_name
      }
    },
    None => {
      if option_values.interactive {
        ask(NAME_PROMPT, NAME_DEFAULT)
      } else {
        NAME_DEFAULT.to_string()
      }
    },
  };
  format!("Hello, {}!", name)
}

fn show_help() {
  print_help(&HELP_INFO);
}
