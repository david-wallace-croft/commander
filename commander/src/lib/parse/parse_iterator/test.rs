//==============================================================================
//! Unit tests for module parse_iterator
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-07-27
//! - Updated: 2024-07-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_found::ParseFound;
use crate::parse::parse_iterator::ParseIterator;
use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_option_name::ParseOptionName;
use crate::parse::parse_output::ParseOutput;
use crate::parse::value_usage::ValueUsage;

const TEST_ARGS_0: &[&str] = &[
  "-T", "--TEST",
];

const TEST_PARSE_OPTION_CONFIG_0: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_0",
  name: ParseOptionName::Both {
    name_short: 'T',
    name_long: "TEST",
  },
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIGS_0: &[&ParseOptionConfig] =
  &[&TEST_PARSE_OPTION_CONFIG_0];

#[test]
fn test_0() {
  let mut parse_iterator = ParseIterator {
    args: TEST_ARGS_0,
    parse_option_configs: TEST_PARSE_OPTION_CONFIGS_0,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected0: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      // TODO: Should this be 1 since the hyphen is at position zero?
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let expected1: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let expected3: Option<ParseOutput> = None;

  let actual0: Option<ParseOutput> = parse_iterator.next();

  assert_eq!(actual0, expected0);

  let actual1: Option<ParseOutput> = parse_iterator.next();

  assert_eq!(actual1, expected1);

  let actual2: Option<ParseOutput> = parse_iterator.next();

  assert_eq!(actual2, expected3);
}

#[test]
fn test_1() {
  let parse_iterator = ParseIterator {
    args: TEST_ARGS_0,
    parse_option_configs: TEST_PARSE_OPTION_CONFIGS_0,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected0: ParseOutput = ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      // TODO: Should this be 1 since the hyphen is at position zero?
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  };

  let expected1: ParseOutput = ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  };

  let expected: Vec<ParseOutput> = vec![
    expected0, expected1,
  ];

  let mut actual: Vec<ParseOutput> = Vec::new();

  for parse_output in parse_iterator {
    actual.push(parse_output);
  }

  assert_eq!(actual, expected);
}
