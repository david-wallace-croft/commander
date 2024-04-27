//==============================================================================
//! Unit tests for module Parse
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-04-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

const OPTION_CONFIG_2_OPTIONAL: OptionConfig2 = OptionConfig2 {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Optional,
};

const OPTION_CONFIG_2_REQUIRED: OptionConfig2 = OptionConfig2 {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Required,
};

const OPTION_CONFIG_2_VERBOTEN: OptionConfig2 = OptionConfig2 {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Verboten,
};

#[test]
fn test_option_config_2_parse_optional_0() {
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "value".to_string(),
  ];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_OPTIONAL.parse(test_args_slice);

  assert_eq!(Some(Ok(Some("value".into()))), actual_result);
}

#[test]
fn test_option_config_2_parse_optional_1() {
  let test_args_slice: &[String] = &["-T".to_string()];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_OPTIONAL.parse(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_optional_bool_0() {
  let test_args_slice: &[String] = &["-T=false".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_OPTIONAL.parse_bool(test_args_slice);

  assert_eq!(Some(Ok(Some(false))), actual_result);
}

#[test]
fn test_option_config_2_parse_optional_bool_1() {
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "false".to_string(),
  ];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_OPTIONAL.parse_bool(test_args_slice);

  // TODO: Change this so that it runs None if there is no equals (=)
  // assert_eq!(Some(Ok(None)), actual_result);

  assert_eq!(Some(Ok(Some(false))), actual_result);
}

#[test]
fn test_option_config_2_parse_optional_bool_2() {
  let test_args_slice: &[String] = &["-T=invalid".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_OPTIONAL.parse_bool(test_args_slice);

  assert_eq!(Some(Err(CommanderParseError::InvalidValue)), actual_result);
}

#[test]
fn test_option_config_2_parse_required_0() {
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "value".to_string(),
  ];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_REQUIRED.parse(test_args_slice);

  assert_eq!(Some(Ok(Some("value".into()))), actual_result);
}

#[test]
fn test_option_config_2_parse_required_1() {
  let test_args_slice: &[String] = &["-T".to_string()];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_REQUIRED.parse(test_args_slice);

  assert_eq!(
    Some(Err(CommanderParseError::RequiredValueMissing)),
    actual_result
  );
}

#[test]
fn test_option_config_2_parse_required_bool_0() {
  let test_args_slice: &[String] = &["-T=false".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_REQUIRED.parse_bool(test_args_slice);

  assert_eq!(Some(Ok(Some(false))), actual_result);
}

#[test]
fn test_option_config_2_parse_required_bool_1() {
  let test_args_slice: &[String] = &["-T=true".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_REQUIRED.parse_bool(test_args_slice);

  assert_eq!(Some(Ok(Some(true))), actual_result);
}

#[test]
fn test_option_config_2_parse_required_bool_2() {
  let test_args_slice: &[String] = &["-T=invalid".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_REQUIRED.parse_bool(test_args_slice);

  assert_eq!(Some(Err(CommanderParseError::InvalidValue)), actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_0() {
  let test_args_slice: &[String] = &["-T".to_string()];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_1() {
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "value".to_string(),
  ];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_2() {
  let test_args_slice: &[String] = &["-T=value".to_string()];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse(test_args_slice);

  assert_eq!(
    Some(Err(CommanderParseError::VerbotenValuePresent)),
    actual_result
  );
}

#[test]
fn test_option_config_2_parse_verboten_bool_0() {
  let test_args_slice: &[String] = &["-T".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse_bool(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_bool_1() {
  let test_args_slice: &[String] = &["-t".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse_bool(test_args_slice);

  assert_eq!(None, actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_bool_2() {
  let test_args_slice: &[String] = &["--TEST".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse_bool(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_bool_3() {
  let test_args_slice: &[String] = &["--test".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse_bool(test_args_slice);

  assert_eq!(None, actual_result);
}

#[test]
fn test_option_config_2_parse_verboten_bool_4() {
  let test_args_slice: &[String] = &["-TEST".to_string()];

  let actual_result: Option<Result<Option<bool>, CommanderParseError>> =
    OPTION_CONFIG_2_VERBOTEN.parse_bool(test_args_slice);

  // TODO: Make this work; should be Some(Ok(None))
  assert_eq!(None, actual_result);
}

// TODO: left off here

#[test]
fn test_parse_option_type_bool_with_optional_value_16() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-T=invalid".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Err(CommanderParseError::InvalidValue), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_17() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST=invalid".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Err(CommanderParseError::InvalidValue), actual_result);
}

//----------------------------------------------------------------------------
/// Unit test for parse_option_type_bool_with_optional_value()
/// where OptionConfig cannot have a value.
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_bool_with_optional_value_where_cannot_have_value() {
  const ARG_OPTION_TEST_CANNOT_HAVE_VALUE: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Verboten,
  };
  let test_args_slice: &[String] = &["--TEST=true".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST_CANNOT_HAVE_VALUE,
  );
  assert_eq!(Err(CommanderParseError::FunctionIncorrect), actual_result);
}

#[test]
fn test_parse_option_type_string_with_optional_value_0() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "abc".to_string(),
  ];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(Ok(Some(String::from("abc")))), actual_result);
}

#[test]
fn test_parse_option_type_string_with_optional_value_1() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "--TEST".to_string(),
    "abc".to_string(),
  ];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(Ok(Some(String::from("abc")))), actual_result);
}

#[test]
fn test_parse_option_type_string_with_optional_value_2() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-T=abc".to_string()];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(Ok(Some(String::from("abc")))), actual_result);
}

#[test]
fn test_parse_option_type_string_with_optional_value_3() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST=abc".to_string()];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(Ok(Some(String::from("abc")))), actual_result);
}

#[test]
fn test_parse_option_type_string_with_optional_value_4() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-T=".to_string()];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(
    Some(Err(CommanderParseError::OptionalValueMissing)),
    actual_result
  );
}

#[test]
fn test_parse_option_type_string_with_optional_value_5() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST=".to_string()];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(
    Some(Err(CommanderParseError::OptionalValueMissing)),
    actual_result
  );
}

//----------------------------------------------------------------------------
/// Unit test for parse_option_type_bool_with_optional_value()
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_string_with_required_value_where_cannot_have_value() {
  const ARG_OPTION_TEST_CANNOT_HAVE_VALUE: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Verboten,
  };
  let test_args_slice: &[String] = &["--TEST=abc".to_string()];
  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    parse_option_type_string_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST_CANNOT_HAVE_VALUE,
    );
  // TODO: Change this to make this impossible via static typing
  assert_eq!(None, actual_result);
}

#[test]
fn test_parse_unrecognized_long() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let recognized_options: Vec<OptionConfig> = vec![ARG_OPTION_TEST];
  let test_args_slice: &[String] = &["--unrecognized".to_string()];
  let actual_result: Option<Vec<String>> =
    parse_unrecognized(test_args_slice, &recognized_options);
  assert_eq!(Some(vec![String::from("unrecognized")]), actual_result);
}

#[test]
fn test_parse_unrecognized_short() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let recognized_options: Vec<OptionConfig> = vec![ARG_OPTION_TEST];
  let test_args_slice: &[String] = &["-u".to_string()];
  let actual_result: Option<Vec<String>> =
    parse_unrecognized(test_args_slice, &recognized_options);
  assert_eq!(Some(vec![String::from("u")]), actual_result);
}
