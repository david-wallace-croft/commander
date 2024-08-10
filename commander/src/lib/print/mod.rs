//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2024-08-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_found::ParseFound;
use crate::parse::parse_output::ParseOutput;

pub mod app_info;
pub mod help_info;
pub mod option_config;

//------------------------------------------------------------------------------
/// Makes a message about an unknown option.
//------------------------------------------------------------------------------
pub fn format_unknown_option(unknown_option: &ParseOutput) -> String {
  let parse_found: &ParseFound = &unknown_option.found;

  // TODO: char_index for short option names
  format!(
    "Unknown option at position {}: \"{}\"",
    parse_found.get_arg_index(),
    parse_found.get_name(),
  )
}

//------------------------------------------------------------------------------
/// Makes a message about unknown options.
//------------------------------------------------------------------------------
pub fn format_unknown_options(unknown_options: &Vec<ParseOutput>) -> String {
  let mut unknown_options_string: String = String::new();

  for unknown_option in unknown_options {
    let unknown_option_string: String = format_unknown_option(unknown_option);

    unknown_options_string.push_str(&unknown_option_string);
  }

  unknown_options_string
}

//------------------------------------------------------------------------------
/// Prints a message about an unknown option.
//------------------------------------------------------------------------------
pub fn print_unknown_option(unknown_option: &ParseOutput) {
  let unknown_option_string: String = format_unknown_option(unknown_option);

  // TODO: Output to standard error?
  println!("{}", unknown_option_string);
}

//------------------------------------------------------------------------------
/// Prints a message about unknown options.
//------------------------------------------------------------------------------
pub fn print_unknown_options(unknown_options: &Vec<ParseOutput>) {
  let unknown_options_string: String = format_unknown_options(unknown_options);

  println!("{}", unknown_options_string)
}
