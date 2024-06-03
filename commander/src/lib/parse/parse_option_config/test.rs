//==============================================================================
//! Unit tests for module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-02
//! - Updated: 2024-06-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

const TEST_PARSE_OPTION_CONFIG_LONG: ParseOptionConfig = ParseOptionConfig {
  name_long: Some("TEST"),
  name_short: None,
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIG_SHORT: ParseOptionConfig = ParseOptionConfig {
  name_long: None,
  name_short: Some('T'),
  value_usage: ValueUsage::Optional,
};

#[test]
fn test_make_hyphenated_option_name_0() {
  let expected: Option<String> = Some("--TEST".to_string());

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_LONG
    .make_hyphenated_option_name(HyphenationType::Long);

  assert_eq!(expected, actual);
}

#[test]
fn test_make_hyphenated_option_name_1() {
  let expected: Option<String> = None;

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_LONG
    .make_hyphenated_option_name(HyphenationType::Short);

  assert_eq!(expected, actual);
}

#[test]
fn test_make_hyphenated_option_name_2() {
  let expected: Option<String> = None;

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_SHORT
    .make_hyphenated_option_name(HyphenationType::Long);

  assert_eq!(expected, actual);
}

#[test]
fn test_make_hyphenated_option_name_3() {
  let expected: Option<String> = Some("-T".to_string());

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_SHORT
    .make_hyphenated_option_name(HyphenationType::Short);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_hyphenated_option_name_0() {
  let expected: ParseOutput = ParseOutput::default();

  let actual: ParseOutput = ParseOptionConfig::parse_hyphenated_option_name(
    "--UNRECOGNIZED",
    0,
    "--TEST",
    ValueUsage::Optional,
  );

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_hyphenated_option_name_1() {
  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = ParseOptionConfig::parse_hyphenated_option_name(
    "--TEST",
    0,
    "--TEST",
    ValueUsage::Optional,
  );

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_hyphenated_option_name_2() {
  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = ParseOptionConfig::parse_hyphenated_option_name(
    "--TEST=",
    0,
    "--TEST",
    ValueUsage::Optional,
  );

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_hyphenated_option_name_3() {
  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: Some("VALUE".to_string()),
  };

  let actual: ParseOutput = ParseOptionConfig::parse_hyphenated_option_name(
    "--TEST=VALUE",
    0,
    "--TEST",
    ValueUsage::Optional,
  );

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_hyphenated_option_name_4() {
  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::VerbotenValuePresent),
    index: Some(0),
    value: Some("VALUE".to_string()),
  };

  let actual: ParseOutput = ParseOptionConfig::parse_hyphenated_option_name(
    "--TEST=VALUE",
    0,
    "--TEST",
    ValueUsage::Verboten,
  );

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_hyphenated_option_name_5() {
  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::RequiredValueMissing),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = ParseOptionConfig::parse_hyphenated_option_name(
    "--TEST",
    0,
    "--TEST",
    ValueUsage::Required,
  );

  assert_eq!(expected, actual);
}
