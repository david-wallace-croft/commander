//==============================================================================
//! Functions to parse options from command-line arguments
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

#[cfg(test)]
mod test;

use crate::*;
use ::std::collections::HashSet;
use ::std::env;
use std::iter::Skip;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum CommanderParseError {
  InvalidValue,
  OptionalValueMissingAfterEquals,
  RequiredValueMissing,
  VerbotenValuePresent,
}

pub struct ParseInput {
  pub args: Vec<String>,
  pub skip: usize,
}

impl ParseInput {
  pub fn from_slice(args_slice: &[&str]) -> Self {
    let args: Vec<String> =
      args_slice.iter().map(|arg| arg.to_string()).collect();

    Self {
      args,
      skip: 0,
    }
  }

  pub fn new() -> Self {
    let args: Vec<String> = env::args().collect();
    Self {
      args,
      skip: 1,
    }
  }
}

// TODO: return this from parse functions
pub struct ParseOutput {
  pub error: Option<CommanderParseError>,
  pub index: Option<usize>,
  pub value: Option<String>,
}

impl ParseOutput {
  pub fn error(error: CommanderParseError) -> Self {
    Self {
      error: Some(error),
      index: None,
      value: None,
    }
  }
}

// TODO: Return data structure with index of option so value can be parsed
fn parse_hyphenated_option_name_with_optional_value(
  parse_input: &ParseInput,
  hyphenated_option_name: &str,
) -> Option<Result<Option<String>, CommanderParseError>> {
  let hyphenated_option_name_equals: &String =
    &format!("{}=", hyphenated_option_name);

  // TODO: use enum to get the index to return
  for arg in parse_input.args.iter().skip(parse_input.skip) {
    if arg.starts_with(hyphenated_option_name_equals) {
      let value: &str =
        arg.strip_prefix(hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        return Some(Err(CommanderParseError::OptionalValueMissingAfterEquals));
      }

      return Some(Ok(Some(value.to_string())));
    }

    if arg.eq(hyphenated_option_name) {
      return Some(Ok(None));
    }
  }

  None
}

fn parse_hyphenated_option_name_with_required_value(
  parse_input: &ParseInput,
  hyphenated_option_name: &str,
) -> Option<Result<Option<String>, CommanderParseError>> {
  let hyphenated_option_name_equals: String =
    format!("{}=", hyphenated_option_name);

  let mut args_iter: Skip<Iter<String>> =
    parse_input.args.iter().skip(parse_input.skip);

  while let Some(arg) = args_iter.next() {
    if arg.starts_with(&hyphenated_option_name_equals) {
      let value: &str =
        arg.strip_prefix(&hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        return Some(Err(CommanderParseError::OptionalValueMissingAfterEquals));
      }

      return Some(Ok(Some(value.to_string())));
    }

    if !arg.eq(&hyphenated_option_name) {
      continue;
    }

    let next_option: Option<&String> = args_iter.next();

    if next_option.is_none() {
      return Some(Err(CommanderParseError::RequiredValueMissing));
    }

    // TODO: What if it is an option starting with - or --?

    let value: String = next_option.unwrap().clone();

    return Some(Ok(Some(value)));
  }

  None
}

// TODO: Return data structure with index of option so value can be parsed
fn parse_hyphenated_option_name_with_verboten_value(
  parse_input: &ParseInput,
  hyphenated_option_name: &str,
) -> Option<Result<Option<String>, CommanderParseError>> {
  let hyphenated_option_name_equals: String =
    format!("{}=", hyphenated_option_name);

  for arg in parse_input.args.iter().skip(parse_input.skip) {
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
  parse_input: &ParseInput,
  recognized_options: &Vec<OptionConfig>,
) -> Option<Vec<String>> {
  let mut unrecognized_set: HashSet<String> = HashSet::new();

  'outer: for arg in parse_input.args.iter().skip(parse_input.skip) {
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

impl OptionConfig<'_> {
  pub fn parse(
    &self,
    parse_input: &ParseInput,
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
          parse_input,
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
          parse_input,
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
    parse_input: &ParseInput,
  ) -> Option<Result<Option<bool>, CommanderParseError>> {
    let result_option: Option<Result<Option<String>, CommanderParseError>> =
      self.parse(parse_input);

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
    parse_input: &ParseInput,
    default: bool,
  ) -> Result<bool, CommanderParseError> {
    let value_option_result_option: Option<
      Result<Option<bool>, CommanderParseError>,
    > = self.parse_bool(parse_input);

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
