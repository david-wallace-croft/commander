//==============================================================================
//! Unit tests for module Parse
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

const OPTION_CONFIG_NAMELESS: OptionConfig = OptionConfig {
  brief_description: None,
  name_long: None,
  name_short: None,
  value_usage: ValueUsage::Optional,
};

const OPTION_CONFIG_OPTIONAL: OptionConfig = OptionConfig {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Optional,
};

const OPTION_CONFIG_REQUIRED: OptionConfig = OptionConfig {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Required,
};

const OPTION_CONFIG_VERBOTEN: OptionConfig = OptionConfig {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Verboten,
};

#[test]
fn test_option_config_parse_optional_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_3() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--TEST", "value",
  ]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: ParseOutput = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_8() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::OptionConfigNameless),
    index: None,
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_NAMELESS.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_bool_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_OPTIONAL
    .parse(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_bool_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "false",
  ]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_OPTIONAL
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_bool_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::InvalidValue);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_OPTIONAL
    .parse(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_optional_bool_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=invalid"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::InvalidValue);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_OPTIONAL
    .parse(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected = ParseOutput {
    error: Some(CommanderParseError::RequiredValueMissing),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected = ParseOutput {
    error: Some(CommanderParseError::RequiredValueMissing),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_bool_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_REQUIRED
    .parse(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_bool_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_REQUIRED
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_bool_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::InvalidValue);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_REQUIRED
    .parse(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_multiple_0() {
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

  let actual: ParseOutput = OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_multiple_1() {
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

  let actual: ParseOutput = OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_required_multiple_2() {
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

  let actual: ParseOutput = OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let expected = ParseOutput {
    error: Some(CommanderParseError::VerbotenValuePresent),
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--TEST", "value",
  ]);

  let expected = ParseOutput {
    error: None,
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::VerbotenValuePresent),
    index: Some(0),
    value: Some("value".to_string()),
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_6() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let expected: ParseOutput = ParseOutput {
    error: Some(CommanderParseError::ValueMissingAfterEquals),
    index: Some(0),
    value: None,
  };

  let actual: ParseOutput = OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(true);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_1() {
  let test_parse_input = &ParseInput::from_slice(&["-t"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_2() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: Result<bool, CommanderParseError> = Ok(true);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_3() {
  let test_parse_input = &ParseInput::from_slice(&["--test"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_4() {
  let test_parse_input = &ParseInput::from_slice(&["-TEST"]);

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(false);

  // TODO: Make this work; should be true
  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_5() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::VerbotenValuePresent);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_option_config_parse_verboten_bool_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=true"]);

  let expected: Result<bool, CommanderParseError> =
    Err(CommanderParseError::VerbotenValuePresent);

  let actual: Result<bool, CommanderParseError> = OPTION_CONFIG_VERBOTEN
    .parse(test_parse_input)
    .to_bool_result(false);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_unrecognized_long() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };

  let recognized_options: Vec<OptionConfig> = vec![ARG_OPTION_TEST];

  let test_parse_input = &ParseInput::from_slice(&["--unrecognized"]);

  let expected: Vec<String> = vec![String::from("unrecognized")];

  let actual: Vec<String> =
    parse_unrecognized(test_parse_input, &recognized_options);

  assert_eq!(expected, actual);
}

#[test]
fn test_parse_unrecognized_short() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };

  let recognized_options: Vec<OptionConfig> = vec![ARG_OPTION_TEST];

  let test_parse_input = &ParseInput::from_slice(&["-u"]);

  let expected: Vec<String> = vec![String::from("u")];

  let actual: Vec<String> =
    parse_unrecognized(test_parse_input, &recognized_options);

  assert_eq!(expected, actual);
}
