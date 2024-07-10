//==============================================================================
//! Unit tests for module parse_output
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-29
//! - Updated: 2024-07-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

#[test]
fn test_to_bool_result_0() {
  let test_parse_output = ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: String::new(),
    },
    known: true,
    value: None,
  };

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = test_parse_output.to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_1() {
  let test_parse_output = ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: String::new(),
    },
    known: true,
    value: Some("0".to_string()),
  };

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = test_parse_output.to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_2() {
  let test_parse_output = ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: String::new(),
    },
    known: true,
    value: Some("1".to_string()),
  };

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = test_parse_output.to_bool_result();

  assert_eq!(actual, expected);
}
