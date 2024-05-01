//==============================================================================
//! Functions to parse options from command-line arguments
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

use crate::*;
use ::std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum CommanderParseError {
  InvalidValue,
  OptionalValueMissing,
  RequiredValueMissing,
  VerbotenValuePresent,
}

// TODO: Return data structure with index of option so value can be parsed
fn parse_hyphenated_option_name_with_optional_value(
  args_slice: &[String],
  hyphenated_option_name: &str,
) -> Option<Result<Option<String>, CommanderParseError>> {
  let hyphenated_option_name_equals: String =
    format!("{}=", hyphenated_option_name);

  let length: usize = args_slice.len();

  for index in 0..length {
    let arg: &String = &args_slice[index];

    if arg.starts_with(&hyphenated_option_name_equals) {
      let value: &str =
        arg.strip_prefix(&hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        return Some(Err(CommanderParseError::OptionalValueMissing));
      }

      return Some(Ok(Some(value.to_string())));
    }

    if !arg.eq(&hyphenated_option_name) {
      continue;
    }

    // TODO: Should we support values without equals?

    if index == length - 1 {
      return Some(Ok(None));
    }

    let value: &String = &args_slice[index + 1];

    // TODO: What if it is an unrelated argument?

    return Some(Ok(Some(value.clone())));
  }

  None
}

fn parse_hyphenated_option_name_with_required_value(
  args_slice: &[String],
  hyphenated_option_name: &str,
) -> Option<Result<Option<String>, CommanderParseError>> {
  let result_option: Option<Result<Option<String>, CommanderParseError>> =
    parse_hyphenated_option_name_with_optional_value(
      args_slice,
      hyphenated_option_name,
    );

  if result_option.is_some() {
    let result: Result<Option<String>, CommanderParseError> =
      result_option.unwrap();

    return Some(to_error_if_missing(result));
  }

  None
}

// TODO: Return data structure with index of option so value can be parsed
fn parse_hyphenated_option_name_with_verboten_value(
  args_slice: &[String],
  hyphenated_option_name: &str,
) -> Option<Result<Option<String>, CommanderParseError>> {
  let hyphenated_option_name_equals: String =
    format!("{}=", hyphenated_option_name);

  for arg in args_slice {
    if arg.starts_with(&hyphenated_option_name_equals) {
      return Some(Err(CommanderParseError::VerbotenValuePresent));
    }

    if arg.eq(&hyphenated_option_name) {
      return Some(Ok(None));
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

        if recognized_option.value_usage != ValueUsage::Verboten {
          let name_long_equals: String = format!("{name_long}=");

          if option_name.starts_with(&name_long_equals) {
            continue 'outer;
          }
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

      if recognized_option.value_usage != ValueUsage::Verboten {
        let name_short_string_equals: String = format!("{name_short_string}=");

        if option_name.starts_with(&name_short_string_equals) {
          continue 'outer;
        }
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

fn to_error_if_missing(
  result: Result<Option<String>, CommanderParseError>
) -> Result<Option<String>, CommanderParseError> {
  let option_value_option: Option<String> = result?;

  if option_value_option.is_some() {
    return Ok(option_value_option);
  }

  Err(CommanderParseError::RequiredValueMissing)
}

impl OptionConfig<'_> {
  pub fn parse(
    &self,
    args_slice: &[String],
  ) -> Option<Result<Option<String>, CommanderParseError>> {
    let parse_hyphenated_option_name_function = match self.value_usage {
      ValueUsage::Optional => parse_hyphenated_option_name_with_optional_value,
      ValueUsage::Required => parse_hyphenated_option_name_with_required_value,
      ValueUsage::Verboten => parse_hyphenated_option_name_with_verboten_value,
    };

    if self.name_short.is_some() {
      let arg_option_name_short = self.name_short.unwrap();

      let hyphenated_option_name: String =
        format!("-{}", arg_option_name_short);

      let result_option: Option<Result<Option<String>, CommanderParseError>> =
        parse_hyphenated_option_name_function(
          args_slice,
          &hyphenated_option_name,
        );

      if result_option.is_some() {
        return result_option;
      }
    }

    if self.name_long.is_some() {
      let arg_option_name_long: &str = self.name_long.unwrap();

      let hyphenated_option_name: String =
        format!("--{}", arg_option_name_long);

      let result_option: Option<Result<Option<String>, CommanderParseError>> =
        parse_hyphenated_option_name_function(
          args_slice,
          &hyphenated_option_name,
        );

      if result_option.is_some() {
        return result_option;
      }
    }

    None
  }

  pub fn parse_bool(
    &self,
    args_slice: &[String],
  ) -> Option<Result<Option<bool>, CommanderParseError>> {
    let result_option: Option<Result<Option<String>, CommanderParseError>> =
      self.parse(args_slice);

    let result: Result<Option<String>, CommanderParseError> = result_option?;

    if let Err(error) = result {
      return Some(Err(error));
    }

    let value_option: Option<String> = result.unwrap();

    if value_option.is_none() {
      return Some(Ok(None));
    }

    let value = value_option.unwrap();

    let lowercase_value: String = value.to_lowercase();

    match lowercase_value.as_str() {
      "0" | "f" | "false" | "n" | "no" | "off" => Some(Ok(Some(false))),
      "1" | "on" | "t" | "true" | "y" | "yes" => Some(Ok(Some(true))),
      _ => Some(Err(CommanderParseError::InvalidValue)),
    }
  }

  pub fn parse_bool_default(
    &self,
    args_slice: &[String],
    default: bool,
  ) -> Result<bool, CommanderParseError> {
    let value_option_result_option: Option<
      Result<Option<bool>, CommanderParseError>,
    > = self.parse_bool(args_slice);

    if value_option_result_option.is_none() {
      return Ok(default);
    }

    let value_option_result: Result<Option<bool>, CommanderParseError> =
      value_option_result_option.unwrap();

    let value_option: Option<bool> = value_option_result?;

    if let Some(value) = value_option {
      return Ok(value);
    }

    Ok(true)
  }
}
