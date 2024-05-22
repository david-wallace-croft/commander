//==============================================================================
//! Functions to parse options from command-line arguments
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-22
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

use ::std::collections::HashSet;
use ::std::env;

//------------------------------------------------------------------------------
/// Whether a option value is optional, required, or verboten (forbidden)
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueUsage {
  Optional,
  Required,
  Verboten,
}

//------------------------------------------------------------------------------
/// Errors that can occur when parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum CommanderParseError {
  InvalidValue,
  ParseConfigNameless,
  RequiredValueMissing,
  ValueMissingAfterEquals,
  VerbotenValuePresent,
}

//------------------------------------------------------------------------------
/// Option configuration metadata for parsing
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct ParseConfig<'a> {
  // TODO: Static compile check to make sure at least one of the names is Some
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
  pub value_usage: ValueUsage,
}

//------------------------------------------------------------------------------
/// The input to parsing an option from the command-line arguments
//------------------------------------------------------------------------------
pub struct ParseInput {
  /// The command-line arguments
  pub args: Vec<String>,
  /// How many command-line arguments to skip before searching for an option
  pub skip: usize,
}

impl ParseInput {
  /// A slice of the command-line arguments with a skip of zero
  pub fn from_slice(args_slice: &[&str]) -> Self {
    let args: Vec<String> =
      args_slice.iter().map(|arg| arg.to_string()).collect();

    Self {
      args,
      skip: 0,
    }
  }
}

impl Default for ParseInput {
  /// The command-line arguments with a skip of one
  fn default() -> Self {
    Self {
      args: env::args().collect(),
      skip: 1,
    }
  }
}

//------------------------------------------------------------------------------
/// The output of parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Debug, Default, PartialEq)]
pub struct ParseOutput {
  pub error: Option<CommanderParseError>,
  // TODO: Might use a 2nd index for multiple short names in a single argument
  pub index: Option<usize>,
  // TODO: Does this need to be OsString?
  pub value: Option<String>,
}

fn parse_hyphenated_option_name_with_optional_value(
  parse_input: &ParseInput,
  hyphenated_option_name: &str,
) -> ParseOutput {
  let hyphenated_option_name_equals: &String =
    &format!("{}=", hyphenated_option_name);

  for (arg_index, arg) in
    parse_input.args.iter().enumerate().skip(parse_input.skip)
  {
    if arg.starts_with(hyphenated_option_name_equals) {
      let value: &str =
        arg.strip_prefix(hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        return ParseOutput {
          error: Some(CommanderParseError::ValueMissingAfterEquals),
          index: Some(arg_index),
          value: None,
        };
      }

      return ParseOutput {
        error: None,
        index: Some(arg_index),
        value: Some(value.to_string()),
      };
    }

    if arg.eq(hyphenated_option_name) {
      return ParseOutput {
        error: None,
        index: Some(arg_index),
        value: None,
      };
    }
  }

  ParseOutput::default()
}

fn parse_hyphenated_option_name_with_required_value(
  parse_input: &ParseInput,
  hyphenated_option_name: &str,
) -> ParseOutput {
  let ParseOutput {
    error,
    index,
    value,
  } = parse_hyphenated_option_name_with_optional_value(
    parse_input,
    hyphenated_option_name,
  );

  if error.is_some() || index.is_none() || value.is_some() {
    return ParseOutput {
      error,
      index,
      value,
    };
  }

  ParseOutput {
    error: Some(CommanderParseError::RequiredValueMissing),
    index,
    value,
  }
}

fn parse_hyphenated_option_name_with_verboten_value(
  parse_input: &ParseInput,
  hyphenated_option_name: &str,
) -> ParseOutput {
  let ParseOutput {
    error,
    index,
    value,
  } = parse_hyphenated_option_name_with_optional_value(
    parse_input,
    hyphenated_option_name,
  );

  if error.is_some() || index.is_none() || value.is_none() {
    return ParseOutput {
      error,
      index,
      value,
    };
  }

  ParseOutput {
    error: Some(CommanderParseError::VerbotenValuePresent),
    index,
    value,
  }
}

//------------------------------------------------------------------------------
/// Returns a list of unrecognized options from the command-line arguments
//------------------------------------------------------------------------------
pub fn parse_unrecognized(
  parse_input: &ParseInput,
  recognized_options: &Vec<ParseConfig>,
) -> Vec<String> {
  let mut unrecognized_set: HashSet<String> = HashSet::new();

  'outer: for arg in parse_input.args.iter().skip(parse_input.skip) {
    let (prefix, using_long_name) = if arg.starts_with("--") {
      ("--", true)
    } else if arg.starts_with('-') {
      ("-", false)
    } else {
      continue;
    };

    let arg_option_name: &str = arg.strip_prefix(prefix).unwrap();

    if arg_option_name.eq("") {
      unrecognized_set.insert(String::from(""));

      continue;
    }

    for recognized_option in recognized_options {
      let recognized_option_name: String = if using_long_name {
        if recognized_option.name_long.is_none() {
          continue;
        }

        recognized_option.name_long.unwrap().to_string()
      } else {
        if recognized_option.name_short.is_none() {
          continue;
        }

        recognized_option.name_short.unwrap().to_string()
      };

      if arg_option_name.eq(&recognized_option_name) {
        continue 'outer;
      }

      let recognized_option_name_equals: String =
        format!("{recognized_option_name}=");

      if arg_option_name.starts_with(&recognized_option_name_equals) {
        continue 'outer;
      }
    }

    unrecognized_set.insert(String::from(arg_option_name));
  }

  Vec::from_iter(unrecognized_set)
}

impl ParseConfig<'_> {
  pub fn parse(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    if self.name_short.is_none() && self.name_long.is_none() {
      return ParseOutput {
        error: Some(CommanderParseError::ParseConfigNameless),
        index: None,
        value: None,
      };
    }

    let parse_hyphenated_option_name_function = match self.value_usage {
      ValueUsage::Optional => parse_hyphenated_option_name_with_optional_value,
      ValueUsage::Required => parse_hyphenated_option_name_with_required_value,
      ValueUsage::Verboten => parse_hyphenated_option_name_with_verboten_value,
    };

    // TODO: should check for both short and long name simultaneously

    if self.name_short.is_some() {
      let arg_option_name_short = self.name_short.unwrap();

      let hyphenated_option_name: String =
        format!("-{}", arg_option_name_short);

      let parse_output: ParseOutput = parse_hyphenated_option_name_function(
        parse_input,
        &hyphenated_option_name,
      );

      if parse_output.index.is_some() {
        return parse_output;
      }
    }

    if self.name_long.is_some() {
      let arg_option_name_long: &str = self.name_long.unwrap();

      let hyphenated_option_name: String =
        format!("--{}", arg_option_name_long);

      let parse_output: ParseOutput = parse_hyphenated_option_name_function(
        parse_input,
        &hyphenated_option_name,
      );

      if parse_output.index.is_some() {
        return parse_output;
      }
    }

    ParseOutput::default()
  }
}

impl ParseOutput {
  //----------------------------------------------------------------------------
  /// Converts the ParseOutput to a boolean value
  ///
  /// - Returns the error if the error is Some
  /// - Returns the default boolean value if the option index is None
  /// - Returns true if the option value is None
  /// - Returns false if the option value is 0, f, false, n, no, or off
  /// - Returns true if the option value is 1, on, t, true, y, or yes
  /// - Returns an InvalidValue error if the option value is anything else
  //----------------------------------------------------------------------------
  pub fn to_bool_result(
    self,
    default: bool,
  ) -> Result<bool, CommanderParseError> {
    if let Some(error) = self.error {
      return Err(error);
    }

    if self.index.is_none() {
      return Ok(default);
    }

    if self.value.is_none() {
      return Ok(true);
    }

    let lowercase_value: String = self.value.unwrap().to_lowercase();

    match lowercase_value.as_str() {
      "0" | "f" | "false" | "n" | "no" | "off" => Ok(false),
      "1" | "on" | "t" | "true" | "y" | "yes" => Ok(true),
      _ => Err(CommanderParseError::InvalidValue),
    }
  }
}
