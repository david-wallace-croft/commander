//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2024-07-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_input::ParseUnrecognizedOutput;

pub mod app_info;
pub mod help_info;
pub mod option_config;

//------------------------------------------------------------------------------
/// Prints a message about an unrecognized option.
//------------------------------------------------------------------------------
pub fn print_unrecognized_option(
  unrecognized_option: &ParseUnrecognizedOutput
) {
  // TODO: Output to standard error?
  println!(
    "Unrecognized option at position {}: \"{}\"",
    unrecognized_option.index, unrecognized_option.name
  );
}

//------------------------------------------------------------------------------
/// Prints a message about unrecognized options.
//------------------------------------------------------------------------------
pub fn print_unrecognized_options(
  unrecognized_options: &Vec<ParseUnrecognizedOutput>
) {
  for unrecognized_option in unrecognized_options {
    print_unrecognized_option(unrecognized_option);
  }
}
