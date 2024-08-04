//==============================================================================
//! Unit tests for ParseInput.
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-08-04
//! - Updated: 2024-08-04
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use std::string::ToString;
use std::sync::LazyLock;

use crate::parse::parse_option_name::ParseOptionName;
use crate::parse::value_usage::ValueUsage;

use super::*;

static TEST_ARGS_0: LazyLock<Vec<String>> = LazyLock::new(|| {
  [
    "-T", "--TEST",
  ]
  .iter()
  .map(|s| s.to_string())
  .collect()
});

const TEST_ID_0: &str = "TEST_ID_0";

const TEST_PARSE_OPTION_CONFIG_0: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_0,
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIGS_0: &[&ParseOptionConfig] =
  &[&TEST_PARSE_OPTION_CONFIG_0];

static TEST_PARSE_INPUT_0: LazyLock<ParseInput> =
  LazyLock::new(|| ParseInput {
    args: &TEST_ARGS_0,
    known_option_configs: TEST_PARSE_OPTION_CONFIGS_0,
  });

#[test]
fn test0() {
  let expected: ParseIterator = ParseIterator {
    args: &TEST_ARGS_0,
    known_option_configs: TEST_PARSE_OPTION_CONFIGS_0,
    skip_arg: 0,
    skip_char: 0,
  };

  let actual: ParseIterator = TEST_PARSE_INPUT_0.into_iter();

  assert_eq!(actual, expected);
}
