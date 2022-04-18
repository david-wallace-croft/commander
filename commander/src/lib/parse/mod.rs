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

#[cfg(test)]
mod test;

use crate::*;

//------------------------------------------------------------------------------
/// Parses a boolean option that has no value
//------------------------------------------------------------------------------
pub fn parse_option_type_bool_without_value(
  args_slice: &[String],
  arg_option: &OptionConfig,
) -> bool {
  if arg_option.name_short.is_some() {
    let hyphenated_name_short: String =
      format!("-{}", arg_option.name_short.unwrap());
    if args_slice.contains(&hyphenated_name_short) {
      return true;
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long: String =
      format!("--{}", arg_option.name_long.unwrap());
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
    let arg_option_name_short: char = arg_option.name_short.unwrap();
    let hyphenated_name_short: String = format!("-{}", arg_option_name_short);
    let hyphenated_name_short_equals_false: String =
      format!("-{}=false", arg_option_name_short);
    let hyphenated_name_short_equals_true: String =
      format!("-{}=true", arg_option_name_short);
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if arg.eq(&hyphenated_name_short_equals_false) {
        return false;
      }
      if arg.eq(&hyphenated_name_short_equals_true) {
        return true;
      }
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
    let arg_option_name_long: &str = arg_option.name_long.unwrap();
    let hyphenated_name_long: String = format!("--{}", arg_option_name_long);
    let hyphenated_name_long_equals_false: String =
      format!("--{}=false", arg_option_name_long);
    let hyphenated_name_long_equals_true: String =
      format!("--{}=true", arg_option_name_long);
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if arg.eq(&hyphenated_name_long_equals_false) {
        return false;
      }
      if arg.eq(&hyphenated_name_long_equals_true) {
        return true;
      }
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
    let arg_option_name_short = arg_option.name_short.unwrap();
    let hyphenated_name_short: String = format!("-{}", arg_option_name_short);
    let hyphenated_name_short_equals: &str =
      &format!("-{}=", arg_option_name_short);
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if arg.starts_with(hyphenated_name_short_equals) {
        let value: &str = &arg[hyphenated_name_short_equals.len()..];
        return Some(value.to_string());
      }
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
    let arg_option_name_long = arg_option.name_long.unwrap();
    let hyphenated_name_long: String = format!("--{}", arg_option_name_long);
    let hyphenated_name_long_equals: &str =
      &format!("--{}=", arg_option_name_long);
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if arg.starts_with(hyphenated_name_long_equals) {
        let value: &str = &arg[hyphenated_name_long_equals.len()..];
        return Some(value.to_string());
      }
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
