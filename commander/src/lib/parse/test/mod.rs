//==============================================================================
//! Unit tests for module Parse
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-04-02
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

//----------------------------------------------------------------------------
/// Unit test for parse_option_type_bool_without_value()
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_bool_without_value() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    can_have_value: false,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
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
/// Unit test for parse_option_type_bool_with_optional_value()
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_bool_with_optional_value() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    can_have_value: false,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("TEST"),
    name_short: Some('T'),
  };
  let test_args_slice: &[String] = &["-T".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "false".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &["-t".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &[
    "-t".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["--TEST".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &[
    "--TEST".to_string(),
    "false".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &[
    "--TEST".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &["--test".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &[
    "--test".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["-TEST".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &[
    "-TEST".to_string(),
    "true".to_string(),
  ];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["-T=false".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["-T=true".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(true, actual_result);
  let test_args_slice: &[String] = &["--TEST=false".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(false, actual_result);
  let test_args_slice: &[String] = &["--TEST=true".to_string()];
  let actual_result = parse_option_type_bool_with_optional_value(
    test_args_slice,
    &ARG_OPTION_TEST,
  );
  assert_eq!(true, actual_result);
}

//----------------------------------------------------------------------------
/// Unit test for parse_option_type_bool_with_optional_value()
//----------------------------------------------------------------------------
#[test]
fn test_parse_option_type_string_with_required_value() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: None,
    can_have_value: true,
    default_value_bool: false,
    is_type_bool: false,
    name_long: Some("TEST"),
    name_short: Some('T'),
  };
  let test_args_slice: &[String] = &[
    "-T".to_string(),
    "abc".to_string(),
  ];
  let actual_result: Option<String> =
    parse_option_type_string_with_required_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(String::from("abc")), actual_result);
  let test_args_slice: &[String] = &[
    "--TEST".to_string(),
    "abc".to_string(),
  ];
  let actual_result: Option<String> =
    parse_option_type_string_with_required_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(String::from("abc")), actual_result);
  let test_args_slice: &[String] = &["-T=abc".to_string()];
  let actual_result: Option<String> =
    parse_option_type_string_with_required_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(String::from("abc")), actual_result);
  let test_args_slice: &[String] = &["--TEST=abc".to_string()];
  let actual_result: Option<String> =
    parse_option_type_string_with_required_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(String::from("abc")), actual_result);
  let test_args_slice: &[String] = &["-T=".to_string()];
  let actual_result: Option<String> =
    parse_option_type_string_with_required_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(String::from("")), actual_result);
  let test_args_slice: &[String] = &["--TEST=".to_string()];
  let actual_result: Option<String> =
    parse_option_type_string_with_required_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
  assert_eq!(Some(String::from("")), actual_result);
}
