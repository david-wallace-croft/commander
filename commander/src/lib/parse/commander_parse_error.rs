//==============================================================================
//! Module for CommanderParseError
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-06-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

//------------------------------------------------------------------------------
/// Errors that can occur when parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum CommanderParseError {
  InvalidValue,
  RequiredValueMissing,
  ValueMissingAfterEquals,
  VerbotenValuePresent,
}
