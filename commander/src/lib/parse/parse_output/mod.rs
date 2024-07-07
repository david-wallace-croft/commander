//==============================================================================
//! Module for ParseOutput
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-07-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::parse_error::ParseError;

#[cfg(test)]
mod test;

// TODO: Move this to its own module; maybe add function get_arg_index(&self)
#[derive(Clone, Debug, PartialEq)]
pub enum ParseFound {
  Long {
    arg_index: usize,
    name_long: String,
  },
  Short {
    arg_index: usize,
    char_index: usize,
    name_short: char,
  },
}

//------------------------------------------------------------------------------
/// The output of parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct ParseOutput {
  pub error: Option<ParseError>,
  pub found: ParseFound,
  // TODO: Does this need to be OsString?
  pub value: Option<String>,
}

// TODO: Maybe use Visitor on different option types after making a ParseOutput

impl ParseOutput {
  //----------------------------------------------------------------------------
  /// Converts the ParseOutput to a boolean value
  ///
  /// - Returns the error if the error is Some
  /// - Returns true if the option value is None
  /// - Returns false if the option value is 0, f, false, n, no, or off
  /// - Returns true if the option value is 1, on, t, true, y, or yes
  /// - Returns an InvalidValue error if the option value is anything else
  //----------------------------------------------------------------------------
  pub fn to_bool_result(self) -> Result<bool, ParseError> {
    if let Some(error) = self.error {
      return Err(error);
    }

    if self.value.is_none() {
      return Ok(true);
    }

    let lowercase_value: String = self.value.unwrap().to_lowercase();

    match lowercase_value.as_str() {
      "0" | "f" | "false" | "n" | "no" | "off" => Ok(false),
      "1" | "on" | "t" | "true" | "y" | "yes" => Ok(true),
      _ => Err(ParseError::InvalidValue),
    }
  }
}
