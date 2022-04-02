//==============================================================================
//! Functions to parse options from command-line arguments
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-04-02
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

use crate::*;

//------------------------------------------------------------------------------
/// Parses an option that has no value
//------------------------------------------------------------------------------
pub fn parse_option_type_bool_without_value(
  args_slice: &[String],
  arg_option: &OptionConfig,
) -> bool {
  if arg_option.name_short.is_some() {
    let hyphenated_name_short = format!("-{}", arg_option.name_short.unwrap());
    if args_slice.contains(&hyphenated_name_short) {
      return true;
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long = format!("--{}", arg_option.name_long.unwrap());
    if args_slice.contains(&hyphenated_name_long) {
      return true;
    }
  }
  arg_option.default_value_bool
}

//------------------------------------------------------------------------------
/// Parses an option that has an optional boolean value
//------------------------------------------------------------------------------
pub fn parse_option_type_bool_with_optional_value(
  args_slice: &[String],
  arg_option: &OptionConfig,
) -> bool {
  let length: usize = args_slice.len();
  if arg_option.name_short.is_some() {
    let hyphenated_name_short: String =
      format!("-{}", arg_option.name_short.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_short) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        if value.eq("false") {
          return false;
        }
      }
      return true;
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long: String =
      format!("--{}", arg_option.name_long.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_long) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        if value.eq("false") {
          return false;
        }
      }
      return true;
    }
  }
  arg_option.default_value_bool
}

//------------------------------------------------------------------------------
/// Parses an option that requires a string value
//------------------------------------------------------------------------------
// TODO: Can we return a string slice instead of a String?
pub fn parse_option_type_string_with_required_value(
  args_slice: &[String],
  arg_option: &OptionConfig,
) -> Option<String> {
  let length: usize = args_slice.len();
  if arg_option.name_short.is_some() {
    let hyphenated_name_short: String =
      format!("-{}", arg_option.name_short.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_short) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        // TODO: What if value starts with a hyphen?
        return Some(value.to_string());
      } else {
        return None;
      }
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long: String =
      format!("--{}", arg_option.name_long.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_long) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        // TODO: What if value starts with a hyphen?
        return Some(value.to_string());
      } else {
        return None;
      }
    }
  }
  None
}

#[cfg(test)]
mod tests {

  use super::*;

  //----------------------------------------------------------------------------
  /// placeholder
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
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &["-t".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &["--TEST".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &["--test".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &["-TEST".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(false, actual_result);
  }

  //----------------------------------------------------------------------------
  /// placeholder
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
  }
}