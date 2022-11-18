//==============================================================================
//! Functions to parse options from command-line arguments
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Version: 2022-11-18
//! - Since: 2022-04-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

use crate::*;
use std::collections::HashSet;

//------------------------------------------------------------------------------
/// Parses a boolean option that has no value
//------------------------------------------------------------------------------
pub fn parse_option_type_bool_without_value(
  args_slice: &[String],
  option_config: &OptionConfig,
) -> bool {
  if option_config.name_short.is_some() {
    let hyphenated_name_short: String =
      format!("-{}", option_config.name_short.unwrap());
    if args_slice.contains(&hyphenated_name_short) {
      return true;
    }
  }
  if option_config.name_long.is_some() {
    let hyphenated_name_long: String =
      format!("--{}", option_config.name_long.unwrap());
    if args_slice.contains(&hyphenated_name_long) {
      return true;
    }
  }
  option_config.default_value_bool
}

//------------------------------------------------------------------------------
/// Parses an option that has an optional boolean value
//------------------------------------------------------------------------------
pub fn parse_option_type_bool_with_optional_value(
  args_slice: &[String],
  option_config: &OptionConfig,
) -> bool {
  let length: usize = args_slice.len();
  if option_config.name_short.is_some() {
    let arg_option_name_short: char = option_config.name_short.unwrap();
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
  if option_config.name_long.is_some() {
    let arg_option_name_long: &str = option_config.name_long.unwrap();
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
  option_config.default_value_bool
}

//------------------------------------------------------------------------------
/// Parses an option that requires a string value
//------------------------------------------------------------------------------
// TODO: Can we return a string slice instead of a String?
pub fn parse_option_type_string_with_required_value(
  args_slice: &[String],
  option_config: &OptionConfig,
) -> Option<String> {
  let length: usize = args_slice.len();
  if option_config.name_short.is_some() {
    let arg_option_name_short = option_config.name_short.unwrap();
    let hyphenated_name_short: String = format!("-{}", arg_option_name_short);
    let hyphenated_name_short_equals: &str =
      &format!("-{}=", arg_option_name_short);
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if arg.starts_with(hyphenated_name_short_equals) {
        let value: &str =
          arg.strip_prefix(hyphenated_name_short_equals).unwrap();
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
  if option_config.name_long.is_some() {
    let arg_option_name_long = option_config.name_long.unwrap();
    let hyphenated_name_long: String = format!("--{}", arg_option_name_long);
    let hyphenated_name_long_equals: &str =
      &format!("--{}=", arg_option_name_long);
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if arg.starts_with(hyphenated_name_long_equals) {
        let value: &str =
          arg.strip_prefix(hyphenated_name_long_equals).unwrap();
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

//------------------------------------------------------------------------------
/// Parses unrecognized options from the arguments.
//------------------------------------------------------------------------------
pub fn parse_unrecognized(
  args_slice: &[String],
  recognized_options: &Vec<OptionConfig>,
) -> Option<Vec<String>> {
  let mut unrecognized_set: HashSet<String> = HashSet::new();
  'outer: for arg in args_slice {
    if !arg.starts_with('-') {
      continue;
    }
    if arg.starts_with("--") {
      let option_name: &str = arg.strip_prefix("--").unwrap();
      if option_name.eq("") {
        unrecognized_set.insert(String::from(""));
        continue;
      }
      for recognized_option in recognized_options {
        if recognized_option.name_long.is_none() {
          continue;
        }
        let name_long: &str = recognized_option.name_long.unwrap();
        if option_name.eq(name_long) {
          continue 'outer;
        }
      }
      unrecognized_set.insert(String::from(option_name));
      continue;
    }
    let option_name: &str = arg.strip_prefix('-').unwrap();
    if option_name.eq("") {
      unrecognized_set.insert(String::from(""));
      continue;
    }
    for recognized_option in recognized_options {
      if recognized_option.name_short.is_none() {
        continue;
      }
      let name_short: char = recognized_option.name_short.unwrap();
      let name_short_string: String = name_short.to_string();
      if option_name.eq(&name_short_string) {
        continue 'outer;
      }
    }
    unrecognized_set.insert(String::from(option_name));
  }
  if unrecognized_set.is_empty() {
    return None;
  }
  let unrecognized_vector: Vec<String> = Vec::from_iter(unrecognized_set);
  Some(unrecognized_vector)
}
