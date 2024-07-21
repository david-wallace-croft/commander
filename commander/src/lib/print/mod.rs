//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2024-07-21
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
/// Prints a message about an unknown option.
//------------------------------------------------------------------------------
pub fn print_unknown_option(unknown_option: &ParseOutput) {
  let parse_found: &ParseFound = &unknown_option.found;

  // TODO: char_index for short option names
  // TODO: Output to standard error?
  println!(
    "Unknown option at position {}: \"{}\"",
    parse_found.get_arg_index(),
    parse_found.get_name(),
  );
}

//------------------------------------------------------------------------------
/// Prints a message about unknown options.
//------------------------------------------------------------------------------
pub fn print_unknown_options(unknown_options: &Vec<ParseOutput>) {
  for unknown_option in unknown_options {
    print_unknown_option(unknown_option);
  }
}
