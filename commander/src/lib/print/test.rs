//==============================================================================
//! Unit tests for module print
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-08-15
//! - Updated: 2024-08-15
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;
use std::sync::LazyLock;

static TEST_PARSE_OUTPUT_0: LazyLock<ParseOutput> =
  LazyLock::new(|| ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "unknown".to_string(),
    },
    known: None,
    value: None,
  });

const TEST_PARSE_OUTPUT_1: ParseOutput = ParseOutput {
  error: None,
  found: ParseFound::Short {
    arg_index: 1,
    char_index: 2,
    name_short: 'u',
  },
  known: None,
  value: None,
};

#[test]
fn test_format_unknown_option_0() {
  let expected: &str = "Unknown option at argument index 1: \"unknown\"";

  let actual: String = format_unknown_option(&TEST_PARSE_OUTPUT_0);

  assert_eq!(&actual, expected);
}

#[test]
fn test_format_unknown_option_1() {
  let expected: &str =
    "Unknown option at argument index 1 character index 2: 'u'";

  let actual: String = format_unknown_option(&TEST_PARSE_OUTPUT_1);

  assert_eq!(&actual, expected);
}
