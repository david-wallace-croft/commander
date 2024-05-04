//==============================================================================
//! Unit tests for module Parse
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

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

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_optional_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(Some(Ok(Some("value".into()))), actual_result);
}

#[test]
fn test_option_config_parse_optional_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_optional_3() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::OptionalValueMissingAfterEquals)),
    actual_result
  );
}

#[test]
fn test_option_config_parse_optional_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--TEST", "value",
  ]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_optional_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(Some(Ok(Some("value".into()))), actual_result);
}

#[test]
fn test_option_config_parse_optional_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_optional_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::OptionalValueMissingAfterEquals)),
    actual_result
  );
}

#[test]
fn test_option_config_parse_optional_bool_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse_bool(test_parse_input);

  assert_eq!(Some(Ok(Some(false))), actual_result);
}

#[test]
fn test_option_config_parse_optional_bool_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "false",
  ]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse_bool(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_optional_bool_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse_bool(test_parse_input);

  assert_eq!(Some(Err(CommanderParseError::InvalidValue)), actual_result);
}

#[test]
fn test_option_config_parse_optional_bool_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=invalid"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_OPTIONAL.parse_bool(test_parse_input);

  assert_eq!(Some(Err(CommanderParseError::InvalidValue)), actual_result);
}

#[test]
fn test_option_config_parse_required_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(Some(Ok(Some("value".to_string()))), actual_result);
}

#[test]
fn test_option_config_parse_required_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_REQUIRED.parse(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::RequiredValueMissing)),
    actual_result
  );
}

#[test]
fn test_option_config_parse_required_bool_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_REQUIRED.parse_bool(test_parse_input);

  assert_eq!(Some(Ok(Some(false))), actual_result);
}

#[test]
fn test_option_config_parse_required_bool_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_REQUIRED.parse_bool(test_parse_input);

  assert_eq!(Some(Ok(Some(true))), actual_result);
}

#[test]
fn test_option_config_parse_required_bool_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_REQUIRED.parse_bool(test_parse_input);

  assert_eq!(Some(Err(CommanderParseError::InvalidValue)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::VerbotenValuePresent)),
    actual_result
  );
}

#[test]
fn test_option_config_parse_verboten_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--TEST", "value",
  ]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::VerbotenValuePresent)),
    actual_result
  );
}

#[test]
fn test_option_config_parse_verboten_bool_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_bool_1() {
  let test_parse_input = &ParseInput::from_slice(&["-t"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  assert_eq!(None, actual_result);
}

#[test]
fn test_option_config_parse_verboten_bool_2() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_parse_verboten_bool_3() {
  let test_parse_input = &ParseInput::from_slice(&["--test"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  assert_eq!(None, actual_result);
}

#[test]
fn test_option_config_parse_verboten_bool_4() {
  let test_parse_input = &ParseInput::from_slice(&["-TEST"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  // TODO: Make this work; should be Some(Ok(None))
  assert_eq!(None, actual_result);
}

#[test]
fn test_option_config_parse_verboten_bool_5() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::VerbotenValuePresent)),
    actual_result
  );
}

#[test]
fn test_option_config_parse_verboten_bool_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=true"]);

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_VERBOTEN.parse_bool(test_parse_input);

  assert_eq!(
    Some(Err(CommanderParseError::VerbotenValuePresent)),
    actual_result
  );
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
  let actual_result: Option<Vec<String>> =
    parse_unrecognized(test_parse_input, &recognized_options);
  assert_eq!(Some(vec![String::from("unrecognized")]), actual_result);
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
  let actual_result: Option<Vec<String>> =
    parse_unrecognized(test_parse_input, &recognized_options);
  assert_eq!(Some(vec![String::from("u")]), actual_result);
}
