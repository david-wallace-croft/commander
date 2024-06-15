//==============================================================================
//! Unit tests for module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-02
//! - Updated: 2024-06-14
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

const PARSE_OPTION_CONFIG_NAMELESS: ParseOptionConfig = ParseOptionConfig {
  name_long: None,
  name_short: None,
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_OPTIONAL: ParseOptionConfig = ParseOptionConfig {
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_REQUIRED: ParseOptionConfig = ParseOptionConfig {
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Required,
};

const TEST_PARSE_OPTION_CONFIG_SHORT: ParseOptionConfig = ParseOptionConfig {
  name_long: None,
  name_short: Some('T'),
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_VERBOTEN: ParseOptionConfig = ParseOptionConfig {
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Verboten,
};

//------------------------------------------------------------------------------
// make_hyphenated_option_name() unit tests
//------------------------------------------------------------------------------

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

//------------------------------------------------------------------------------
// parse() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_next_optional_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-X", "-T", "value",
  ]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(1),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_3() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--EXCLUDE0",
    "--TEST",
    "value",
    "--EXCLUDE1",
  ]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(1),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_OPTIONAL.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_optional_8() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ParseConfigNameless),
    index: None,
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_NAMELESS.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_required_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected = ParseOutput {
    error: Some(CommanderParseError::RequiredValueMissing),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_required_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected = ParseOutput {
    error: Some(CommanderParseError::RequiredValueMissing),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_required_multiple_0() {
  let test_parse_input = &ParseInput {
    args: vec![
      "-T=0".to_string(),
      "-T=1".to_string(),
    ],
    skip: 0,
  };

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: Some("0".to_string()),
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_required_multiple_1() {
  let test_parse_input = &ParseInput {
    args: vec![
      "-T=0".to_string(),
      "-T=1".to_string(),
    ],
    skip: 1,
  };

  let expected = ParseOutput {
    error: None,
    index: Some(1),
    value: Some("1".to_string()),
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_required_multiple_2() {
  let test_parse_input = &ParseInput {
    args: vec![
      "-T=0".to_string(),
      "-T=1".to_string(),
    ],
    skip: usize::MAX,
  };

  let expected = ParseOutput {
    error: None,
    index: None,
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_unrecognized_long() {
  const ARG_OPTION_TEST: ParseOptionConfig = ParseOptionConfig {
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };

  let recognized_options: Vec<ParseOptionConfig> = vec![ARG_OPTION_TEST];

  let test_parse_input = &ParseInput::from_slice(&["--unrecognized"]);

  let expected: Vec<String> = vec![String::from("unrecognized")];

  let actual: Vec<String> =
    test_parse_input.parse_unrecognized(&recognized_options);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_unrecognized_short() {
  const ARG_OPTION_TEST: ParseOptionConfig = ParseOptionConfig {
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };

  let recognized_options: Vec<ParseOptionConfig> = vec![ARG_OPTION_TEST];

  let test_parse_input = &ParseInput::from_slice(&["-u"]);

  let expected: Vec<String> = vec![String::from("u")];

  let actual: Vec<String> =
    test_parse_input.parse_unrecognized(&recognized_options);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let expected = ParseOutput {
    error: Some(CommanderParseError::VerbotenValuePresent),
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--TEST", "value",
  ]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::VerbotenValuePresent),
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_6() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_next_verboten_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(expected, actual);
}

//------------------------------------------------------------------------------
// parse_hyphenated_option_name() unit tests
//------------------------------------------------------------------------------

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

//------------------------------------------------------------------------------
// to_bool_result() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_to_bool_result_optional_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_OPTIONAL
    .parse_next(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_optional_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "false",
  ]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_OPTIONAL
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_optional_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::InvalidValue);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_OPTIONAL
    .parse_next(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_optional_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=invalid"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::InvalidValue);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_OPTIONAL
    .parse_next(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_required_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_required_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_required_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::InvalidValue);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_1() {
  let test_parse_input = &ParseInput::from_slice(&["-t"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_2() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_3() {
  let test_parse_input = &ParseInput::from_slice(&["--test"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_4() {
  let test_parse_input = &ParseInput::from_slice(&["-TEST"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(false);

  // TODO: Make this work; should be true
  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_5() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::VerbotenValuePresent);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_to_bool_result_verboten_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=true"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::VerbotenValuePresent);

  let actual: Result<bool, CommanderParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}