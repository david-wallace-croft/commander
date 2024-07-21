//==============================================================================
//! CroftSoft Commander library usage example application
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-07-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use std::io::{Error, stdin, Stdin, stdout, Write};

use commander::parse::parse_error::ParseError;
use commander::parse::parse_output::ParseOutput;
use commander::print::print_unknown_options;
use constants::*;

pub mod constants;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct OptionValues {
  pub help_wanted: bool,
  pub interactive: Result<bool, ParseError>,
  pub name_option: Option<String>,
  pub quiet: bool,
  pub unknown: Vec<ParseOutput>,
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
  if option_values.help_wanted {
    HELP_INFO.print();

    return;
  }

  if !option_values.unknown.is_empty() {
    print_unknown_options(&option_values.unknown);

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

  let terminal_punctuation: char = if option_values.quiet {
    '.'
  } else {
    '!'
  };

  format!("Hello, {name}{terminal_punctuation}")
}
