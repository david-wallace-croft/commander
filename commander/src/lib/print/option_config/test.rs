//==============================================================================
//! Unit tests module Print module
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-08-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_found::ParseFound;
use crate::parse::parse_option_name::ParseOptionName;
use crate::parse::value_usage::ValueUsage;

use super::*;

const TEST_ID_0: &str = "TEST_ID_0";
const TEST_ID_1: &str = "TEST_ID_1";

const TEST_NAME_LONG_0: &str = "TEST_NAME_LONG_0";
const TEST_NAME_LONG_1: &str = "TEST_NAME_LONG_ABC_1";

const TEST_OPTION_CONFIG_0: OptionConfig = OptionConfig {
  brief_description: Some("TEST_BRIEF_DESCRIPTION_0"),
  parse_option_config: ParseOptionConfig {
    id: TEST_ID_0,
    name: ParseOptionName::Both {
      name_long: TEST_NAME_LONG_0,
      name_short: '0',
    },
    value_usage: ValueUsage::Required,
  },
};

const TEST_OPTION_CONFIG_1: OptionConfig = OptionConfig {
  brief_description: Some("TEST_BRIEF_DESCRIPTION_1"),
  parse_option_config: ParseOptionConfig {
    id: TEST_ID_1,
    name: ParseOptionName::Both {
      name_long: TEST_NAME_LONG_1,
      name_short: '1',
    },
    value_usage: ValueUsage::Verboten,
  },
};

#[test]
fn test_make_print_option_prefix_0() {
  const EXPECTED: &str = "  -0, --TEST_NAME_LONG_0";

  let actual: String = TEST_OPTION_CONFIG_0.make_print_option_prefix();

  assert_eq!(actual, EXPECTED);
}

#[test]
fn test_make_print_string_0() {
  const EXPECTED: &str = "  -0, --TEST_NAME_LONG_0  TEST_BRIEF_DESCRIPTION_0";

  let prefix_len: usize = TEST_OPTION_CONFIG_0.make_print_option_prefix().len();

  let actual: String = TEST_OPTION_CONFIG_0.make_print_string(prefix_len);

  assert_eq!(actual, EXPECTED);
}

#[test]
fn test_make_print_string_for_slice_0() {
  const EXPECTED: &str =
    "  -0, --TEST_NAME_LONG_0      TEST_BRIEF_DESCRIPTION_0\n  \
       -1, --TEST_NAME_LONG_ABC_1  TEST_BRIEF_DESCRIPTION_1\n";

  let actual: String = OptionConfig::make_print_string_for_slice(&[
    TEST_OPTION_CONFIG_0,
    TEST_OPTION_CONFIG_1,
  ]);

  assert_eq!(actual, EXPECTED);
}

#[test]
fn test_parse_0() {
  let test_args: Vec<String> = vec![
    "-0=A".to_string(),
    "-0=B".to_string(),
  ];

  let expected = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 0,
        name_short: '0',
      },
      known: Some("TEST_ID_0".to_string()),
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 1,
        char_index: 0,
        name_short: '0',
      },
      known: Some("TEST_ID_0".to_string()),
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> = TEST_OPTION_CONFIG_0.parse(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_0() {
  let test_args: Vec<String> = vec![
    "-0=A".to_string(),
    "-0=B".to_string(),
  ];

  let expected = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: '0',
    },
    known: Some("TEST_ID_0".to_string()),
    value: Some("B".to_string()),
  });

  let actual: Option<ParseOutput> = TEST_OPTION_CONFIG_0.parse_last(&test_args);

  assert_eq!(actual, expected);
}
