//==============================================================================
//! CroftSoft Commander library usage example
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-04-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use commander::parse::{parse_unrecognized, CommanderParseError};
use commander::OptionConfig2;
use croftsoft_commander_examples::constants::*;
use croftsoft_commander_examples::OptionValues;
use std::env;

//------------------------------------------------------------------------------
/// Parses the options using Commander and then runs the example application.
//------------------------------------------------------------------------------
fn main() {
  let option_values: OptionValues = parse_option_values_using_commander();

  croftsoft_commander_examples::main(option_values);
}

//------------------------------------------------------------------------------
/// Uses the CroftSoft Commander library to parse the application options
//------------------------------------------------------------------------------
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
pub fn parse_option_values_using_commander() -> OptionValues {
  let args: Vec<String> = env::args().collect();

  // println!("{:?}", args);
  // println!("Args length = {}", length);

  let args_slice: &[String] = &args[1..];

  let help_wanted_result_option: Option<
    Result<Option<bool>, CommanderParseError>,
  > = OPTION_CONFIG_H.parse_bool(args_slice);

  // TODO: move this logic to the parser lib
  let help_wanted: bool = if help_wanted_result_option.is_some() {
    let help_wanted_result: Result<Option<bool>, CommanderParseError> =
      help_wanted_result_option.unwrap();

    if help_wanted_result.is_err() {
      false
    } else {
      let help_wanted_option: Option<bool> = help_wanted_result.unwrap();

      if help_wanted_option.is_some() {
        help_wanted_option.unwrap()
      } else {
        true
      }
    }
  } else {
    false
  };

  let interactive_result_option: Option<
    Result<Option<bool>, CommanderParseError>,
  > = OPTION_CONFIG_I.parse_bool(args_slice);

  // TODO: move this logic to the parser lib
  let interactive: Result<bool, CommanderParseError> =
    if interactive_result_option.is_some() {
      let interactive_result: Result<Option<bool>, CommanderParseError> =
        interactive_result_option.unwrap();

      if interactive_result.is_err() {
        Err(interactive_result.unwrap_err())
      } else {
        let interactive_option: Option<bool> = interactive_result.unwrap();

        if interactive_option.is_some() {
          Ok(interactive_option.unwrap())
        } else {
          Ok(true)
        }
      }
    } else {
      Ok(true)
    };

  // TODO: parse_option_type_string_with_default_value

  let name_option_result_option: Option<
    Result<Option<String>, CommanderParseError>,
  > = OPTION_CONFIG_N.parse(args_slice);

  let name_option: Option<String> = if name_option_result_option.is_some() {
    let name_option_result: Result<Option<String>, CommanderParseError> =
      name_option_result_option.unwrap();

    if name_option_result.is_err() {
      None
    } else {
      name_option_result.unwrap()
    }
  } else {
    None
  };

  let arg_option_vector: Vec<OptionConfig2> = OPTION_CONFIGS.to_vec();

  let unrecognized: Option<Vec<String>> =
    parse_unrecognized(args_slice, &arg_option_vector);

  OptionValues {
    help_wanted,
    interactive,
    name_option,
    unrecognized,
  }
}
