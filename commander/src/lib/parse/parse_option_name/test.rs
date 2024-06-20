//==============================================================================
//! Unit tests for module parse_option_name
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-20
//! - Updated: 2024-06-20
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

const TEST_NAME_LONG: &str = "TEST_NAME_LONG";

const TEST_NAME_SHORT: char = 'T';

const TEST_PARSE_OPTION_NAME_BOTH: ParseOptionName = ParseOptionName::Both {
  name_long: TEST_NAME_LONG,
  name_short: TEST_NAME_SHORT,
};

const TEST_PARSE_OPTION_NAME_LONG: ParseOptionName =
  ParseOptionName::Long(TEST_NAME_LONG);

const TEST_PARSE_OPTION_NAME_SHORT: ParseOptionName =
  ParseOptionName::Short(TEST_NAME_SHORT);

#[test]
fn test_get_name_long_0() {
  let expected: Option<&str> = Some(TEST_NAME_LONG);

  let actual: Option<&str> = TEST_PARSE_OPTION_NAME_BOTH.get_name_long();

  assert_eq!(expected, actual);
}

#[test]
fn test_get_name_long_1() {
  let expected: Option<&str> = Some(TEST_NAME_LONG);

  let actual: Option<&str> = TEST_PARSE_OPTION_NAME_LONG.get_name_long();

  assert_eq!(expected, actual);
}

#[test]
fn test_get_name_long_2() {
  let expected: Option<&str> = None;

  let actual: Option<&str> = TEST_PARSE_OPTION_NAME_SHORT.get_name_long();

  assert_eq!(expected, actual);
}

#[test]
fn test_get_name_short_0() {
  let expected: Option<char> = Some(TEST_NAME_SHORT);

  let actual: Option<char> = TEST_PARSE_OPTION_NAME_BOTH.get_name_short();

  assert_eq!(expected, actual);
}

#[test]
fn test_get_name_short_1() {
  let expected: Option<char> = None;

  let actual: Option<char> = TEST_PARSE_OPTION_NAME_LONG.get_name_short();

  assert_eq!(expected, actual);
}

#[test]
fn test_get_name_short_2() {
  let expected: Option<char> = Some(TEST_NAME_SHORT);

  let actual: Option<char> = TEST_PARSE_OPTION_NAME_SHORT.get_name_short();

  assert_eq!(expected, actual);
}
