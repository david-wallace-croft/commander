//==============================================================================
//! CroftSoft Commander library usage example
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-01-15
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

use commander::parse::{
  parse_option_type_bool_with_optional_value,
  parse_option_type_bool_without_value,
  parse_option_type_string_with_required_value,
};
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
    parse_option_type_bool_without_value(args_slice, &ARG_OPTION_H);
  let interactive: bool =
    parse_option_type_bool_with_optional_value(args_slice, &ARG_OPTION_I);
  // TODO: parse_option_type_string_with_default_value
  let name_option =
    parse_option_type_string_with_required_value(args_slice, &ARG_OPTION_N);
  OptionValues {
    help_wanted,
    interactive,
    name_option,
  }
}
