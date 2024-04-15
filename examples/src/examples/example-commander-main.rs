//==============================================================================
//! CroftSoft Commander library usage example
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-04-15
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use commander::parse::{
  parse_option_type_bool_with_optional_value,
  parse_option_type_bool_without_value,
  parse_option_type_string_with_optional_value, parse_unrecognized,
  CommanderParseError,
};
use commander::OptionConfig;
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

  let help_wanted: bool =
    parse_option_type_bool_without_value(args_slice, &OPTION_CONFIG_H);

  let interactive: Result<bool, CommanderParseError> =
    parse_option_type_bool_with_optional_value(args_slice, &OPTION_CONFIG_I);

  // TODO: parse_option_type_string_with_default_value

  let name_option_result_option: Option<
    Result<Option<String>, CommanderParseError>,
  > =
    parse_option_type_string_with_optional_value(args_slice, &OPTION_CONFIG_N);

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

  let arg_option_vector: Vec<OptionConfig> = OPTION_CONFIGS.to_vec();

  let unrecognized: Option<Vec<String>> =
    parse_unrecognized(args_slice, &arg_option_vector);

  OptionValues {
    help_wanted,
    interactive,
    name_option,
    unrecognized,
  }
}
