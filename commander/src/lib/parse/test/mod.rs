//==============================================================================
//! Unit tests for module Parse
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-04-22
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

const OPTION_CONFIG_2_PROHIBITED: OptionConfig2 = OptionConfig2 {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Prohibited,
};

const OPTION_CONFIG_2_REQUIRED: OptionConfig2 = OptionConfig2 {
  brief_description: None,
  name_long: Some("TEST"),
  name_short: Some('T'),
  value_usage: ValueUsage::Required,
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
fn test_option_config_2_parse_prohibited_0() {
  let test_args_slice: &[String] = &["-T".to_string()];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_PROHIBITED.parse(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_prohibited_1() {
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "value".to_string(),
  ];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_PROHIBITED.parse(test_args_slice);

  assert_eq!(Some(Ok(None)), actual_result);
}

#[test]
fn test_option_config_2_parse_prohibited_2() {
  let test_args_slice: &[String] = &["-T=value".to_string()];

  let actual_result: Option<Result<Option<String>, CommanderParseError>> =
    OPTION_CONFIG_2_PROHIBITED.parse(test_args_slice);

  assert_eq!(
    Some(Err(CommanderParseError::ProhibitedValuePresent)),
    actual_result
  );
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

//----------------------------------------------------------------------------
/// Unit test for parse_option_type_bool_without_value()
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_bool_without_value() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Prohibited,
  };
  let test_args_slice: &[String] = &["-T".to_string()];
  let actual_result: bool =
    parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &["-t".to_string()];
  let actual_result: bool =
    parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["--TEST".to_string()];
  let actual_result: bool =
    parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &["--test".to_string()];
  let actual_result: bool =
    parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["-TEST".to_string()];
  let actual_result: bool =
    parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
  assert_eq!(false, actual_result);
}

//----------------------------------------------------------------------------
/// Unit test for parse_option_type_bool_without_value()
/// where the OptionConfig can have a value.
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_bool_without_value_with_can_have_value() {
  const ARG_OPTION_TEST_CAN_HAVE_VALUE: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST".to_string()];
  let actual_result: bool = parse_option_type_bool_without_value(
    test_args_slice,
    &ARG_OPTION_TEST_CAN_HAVE_VALUE,
  );
  assert_eq!(false, actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_0() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-T".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(true), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_1() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "false".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_2() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(true), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_3() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-t".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_4() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "-t".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_5() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(true), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_6() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "--TEST".to_string(),
    "false".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_7() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "--TEST".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(true), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_8() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--test".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_9() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &[
    "--test".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_10() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-TEST".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  // TODO: This should be unrecognized error
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_11() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };

  let test_args_slice: &[String] = &[
    "-TEST".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  // TODO: This should be unrecognized error
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_12() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-T=false".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_13() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["-T=true".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(true), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_14() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST=false".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(false), actual_result);
}

#[test]
fn test_parse_option_type_bool_with_optional_value_15() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
    value_usage: ValueUsage::Optional,
  };
  let test_args_slice: &[String] = &["--TEST=true".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(Ok(true), actual_result);
}

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
  assert_eq!(Err(CommanderParseError::ValueInvalid), actual_result);
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
  assert_eq!(Err(CommanderParseError::ValueInvalid), actual_result);
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
    value_usage: ValueUsage::Prohibited,
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
    value_usage: ValueUsage::Prohibited,
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
