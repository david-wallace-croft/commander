//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2026-04-17
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_error::ParseError;
use crate::parse::parse_found::ParseFound;
use crate::parse::parse_output::ParseOutput;

pub mod app_info;
pub mod help_info;
pub mod option_config;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Makes a message about an error option.
//------------------------------------------------------------------------------
pub fn format_error_option(error_option: &ParseOutput) -> String {
  let parse_error: ParseError = error_option.error.unwrap();

  let error_message: &'static str = match parse_error {
    ParseError::InvalidValue => "Invalid value",
    ParseError::RequiredValueMissing => "Required value missing",
    ParseError::ValueMissingAfterEquals => "Value missing after equals",
    ParseError::VerbotenValuePresent => "Verboten value present",
  };

  let parse_found: &ParseFound = &error_option.found;

  match parse_found {
    ParseFound::Long {
      arg_index,
      name_long,
    } => format!(
      "{} for option at argument index {}: \"{}\"",
      error_message, arg_index, name_long,
    ),
    ParseFound::Short {
      arg_index,
      char_index,
      name_short,
    } => format!(
      "{} for option at argument index {} character index {}: '{}'",
      error_message, arg_index, char_index, name_short,
    ),
  }
}

//------------------------------------------------------------------------------
/// Makes a message about error options.
//------------------------------------------------------------------------------
pub fn format_error_options(error_options: &Vec<ParseOutput>) -> String {
  let mut error_options_string: String = String::new();

  for error_option in error_options {
    let error_option_string: String = format_error_option(error_option);

    error_options_string.push_str(&error_option_string);
  }

  error_options_string
}

//------------------------------------------------------------------------------
/// Makes a message about an unknown option.
//------------------------------------------------------------------------------
pub fn format_unknown_option(unknown_option: &ParseOutput) -> String {
  let parse_found: &ParseFound = &unknown_option.found;

  match parse_found {
    ParseFound::Long {
      arg_index,
      name_long,
    } => format!(
      "Unknown option at argument index {}: \"{}\"",
      arg_index, name_long,
    ),
    ParseFound::Short {
      arg_index,
      char_index,
      name_short,
    } => format!(
      "Unknown option at argument index {} character index {}: '{}'",
      arg_index, char_index, name_short,
    ),
  }
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
/// Prints a message about options with parsing errors.
//------------------------------------------------------------------------------
pub fn print_error_options(error_options: &Vec<ParseOutput>) {
  let error_options_string: String = format_error_options(error_options);

  println!("{error_options_string}")
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
