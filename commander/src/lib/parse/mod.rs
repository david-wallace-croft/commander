//==============================================================================
//! Functions to parse options from command-line arguments
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-04-23
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
  // TODO: Remove this one
  FunctionIncorrect,
  OptionalValueMissing,
  RequiredValueMissing,
  ValueInvalid,
  VerbotenValuePresent,
}

fn parse_hyphenated_option_name_with_optional_boolean_value(
  args_slice: &[String],
  hyphenated_option_name: &str,
) -> Option<Result<Option<bool>, CommanderParseError>> {
  let result_option: Option<Result<Option<String>, CommanderParseError>> =
    parse_hyphenated_option_name_with_optional_value(
      args_slice,
      hyphenated_option_name,
    );

  result_option.as_ref()?;

  let result: Result<Option<String>, CommanderParseError> =
    result_option.unwrap();

  if result.is_err() {
    let result_error: CommanderParseError = result.unwrap_err();

    return Some(Err(result_error));
  }

  let value_option: Option<String> = result.unwrap();

  if value_option.is_none() {
    return Some(Ok(None));
  }

  let value: String = value_option.unwrap();

  if value.eq("false") {
    return Some(Ok(Some(false)));
  }

  if value.eq("true") {
    return Some(Ok(Some(true)));
  }

  // TODO: What if it is an unrelated argument?

  Some(Err(CommanderParseError::ValueInvalid))
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

// TODO: Return data structure with index of option so value can be parsed
fn parse_hyphenated_option_name_with_prohibited_value(
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

//------------------------------------------------------------------------------
/// Parses a boolean option that has no value
//------------------------------------------------------------------------------
pub fn parse_option_type_bool_without_value(
  args_slice: &[String],
  option_config: &OptionConfig,
) -> bool {
  if option_config.value_usage != ValueUsage::Verboten {
    // TODO: Change function signature such that only an option_config
    // subtype that cannot have a value is passed in.

    // TODO: return FunctionIncorrect
    return false;
  }

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
) -> Result<bool, CommanderParseError> {
  if option_config.value_usage == ValueUsage::Verboten {
    // TODO: Change function signature such that only an option_config
    // subtype that can have a value is passed in.
    return Err(CommanderParseError::FunctionIncorrect);
  }

  // TODO: What if it is a bunch of short options put together, e.g., -abc?

  if option_config.name_short.is_some() {
    let arg_option_name_short: char = option_config.name_short.unwrap();

    let hyphenated_option_name: String = format!("-{}", arg_option_name_short);

    let result_option =
      parse_hyphenated_option_name_with_optional_boolean_value(
        args_slice,
        &hyphenated_option_name,
      );

    if let Some(result) = result_option {
      return to_true_if_not_set(result);
    }
  }

  if option_config.name_long.is_some() {
    let arg_option_name_long: &str = option_config.name_long.unwrap();

    let hyphenated_option_name: String = format!("--{}", arg_option_name_long);

    let result_option: Option<Result<Option<bool>, CommanderParseError>> =
      parse_hyphenated_option_name_with_optional_boolean_value(
        args_slice,
        &hyphenated_option_name,
      );

    if let Some(result) = result_option {
      return to_true_if_not_set(result);
    }
  }

  Ok(option_config.default_value_bool)
}

//------------------------------------------------------------------------------
/// Parses an option that has an optional string value
//------------------------------------------------------------------------------
// TODO: Can we return a string slice instead of a String?
pub fn parse_option_type_string_with_optional_value(
  args_slice: &[String],
  option_config: &OptionConfig,
) -> Option<Result<Option<String>, CommanderParseError>> {
  if option_config.value_usage == ValueUsage::Verboten {
    // TODO: Change function signature such that only an option_config
    // subtype that can have a value is passed in.
    return None;
  }

  if option_config.name_short.is_some() {
    let arg_option_name_short = option_config.name_short.unwrap();

    let hyphenated_option_name: String = format!("-{}", arg_option_name_short);

    let result_option: Option<Result<Option<String>, CommanderParseError>> =
      parse_hyphenated_option_name_with_optional_value(
        args_slice,
        &hyphenated_option_name,
      );

    if result_option.is_some() {
      return result_option;
    }
  }

  if option_config.name_long.is_some() {
    let arg_option_name_long = option_config.name_long.unwrap();

    let hyphenated_option_name: String = format!("--{}", arg_option_name_long);

    let result_option: Option<Result<Option<String>, CommanderParseError>> =
      parse_hyphenated_option_name_with_optional_value(
        args_slice,
        &hyphenated_option_name,
      );

    if result_option.is_some() {
      return result_option;
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

fn to_true_if_not_set(
  result: Result<Option<bool>, CommanderParseError>
) -> Result<bool, CommanderParseError> {
  let option_value_option: Option<bool> = result?;

  if let Some(option_value) = option_value_option {
    return Ok(option_value);
  }

  Ok(true)
}

impl OptionConfig2<'_> {
  pub fn parse(
    &self,
    args_slice: &[String],
  ) -> Option<Result<Option<String>, CommanderParseError>> {
    match self.value_usage {
      ValueUsage::Optional => self.parse_optional(args_slice),
      ValueUsage::Required => self.parse_required(args_slice),
      ValueUsage::Verboten => self.parse_verboten(args_slice),
    }
  }

  fn parse_optional(
    &self,
    args_slice: &[String],
  ) -> Option<Result<Option<String>, CommanderParseError>> {
    if self.name_short.is_some() {
      let arg_option_name_short = self.name_short.unwrap();

      let hyphenated_option_name: String =
        format!("-{}", arg_option_name_short);

      // TODO: Since which function is called here is the only difference,
      //   pass the function in as an argument.
      let result_option: Option<Result<Option<String>, CommanderParseError>> =
        parse_hyphenated_option_name_with_optional_value(
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
        parse_hyphenated_option_name_with_optional_value(
          args_slice,
          &hyphenated_option_name,
        );

      if result_option.is_some() {
        return result_option;
      }
    }

    None
  }

  fn parse_required(
    &self,
    args_slice: &[String],
  ) -> Option<Result<Option<String>, CommanderParseError>> {
    if self.name_short.is_some() {
      let arg_option_name_short = self.name_short.unwrap();

      let hyphenated_option_name: String =
        format!("-{}", arg_option_name_short);

      let result_option: Option<Result<Option<String>, CommanderParseError>> =
        parse_hyphenated_option_name_with_required_value(
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
        parse_hyphenated_option_name_with_required_value(
          args_slice,
          &hyphenated_option_name,
        );

      if result_option.is_some() {
        return result_option;
      }
    }

    None
  }

  fn parse_verboten(
    &self,
    args_slice: &[String],
  ) -> Option<Result<Option<String>, CommanderParseError>> {
    if self.name_short.is_some() {
      let arg_option_name_short = self.name_short.unwrap();

      let hyphenated_option_name: String =
        format!("-{}", arg_option_name_short);

      let result_option: Option<Result<Option<String>, CommanderParseError>> =
        parse_hyphenated_option_name_with_prohibited_value(
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
        parse_hyphenated_option_name_with_prohibited_value(
          args_slice,
          &hyphenated_option_name,
        );

      if result_option.is_some() {
        return result_option;
      }
    }

    None
  }
}
