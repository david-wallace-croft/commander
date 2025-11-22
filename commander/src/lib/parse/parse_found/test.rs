//==============================================================================
//! Unit tests for module parse_found
//!
//! # Metadata
//! - Copyright: &copy; 2024-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-07-12
//! - Updated: 2025-11-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

const TEST_NAME_LONG: &str = "TEST_NAME_LONG";

const TEST_NAME_SHORT: char = 'T';

#[test]
fn test_get_arg_index_0() {
  let test_parse_found: ParseFound = ParseFound::Long {
    arg_index: 1,
    name_long: TEST_NAME_LONG.to_string(),
  };

  let expected: usize = 1;

  let actual: usize = test_parse_found.get_arg_index();

  assert_eq!(actual, expected);
}

#[test]
fn test_get_arg_index_1() {
  let test_parse_found: ParseFound = ParseFound::Short {
    arg_index: 1,
    char_index: 2,
    name_short: 's',
  };

  let expected: usize = 1;

  let actual: usize = test_parse_found.get_arg_index();

  assert_eq!(actual, expected);
}

#[test]
fn test_get_name_0() {
  let test_parse_found: ParseFound = ParseFound::Long {
    arg_index: 1,
    name_long: TEST_NAME_LONG.to_string(),
  };

  let expected: String = TEST_NAME_LONG.to_string();

  let actual: String = test_parse_found.get_name();

  assert_eq!(actual, expected);
}

#[test]
fn test_get_name_1() {
  let test_parse_found: ParseFound = ParseFound::Short {
    arg_index: 1,
    char_index: 2,
    name_short: TEST_NAME_SHORT,
  };

  let expected: String = TEST_NAME_SHORT.to_string();

  let actual: String = test_parse_found.get_name();

  assert_eq!(actual, expected);
}
