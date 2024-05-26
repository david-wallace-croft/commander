//==============================================================================
//! Functions to parse options from command-line arguments
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-26
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

use ::std::collections::HashSet;
use ::std::env;

enum HyphenationType {
  Long,
  Short,
}

impl HyphenationType {
  fn determine_hyphenation_type(arg: &str) -> Option<Self> {
    if arg.starts_with("--") {
      Some(Self::Long)
    } else if arg.starts_with('-') {
      Some(Self::Short)
    } else {
      None
    }
  }
}

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
pub struct ParseOptionConfig<'a> {
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

  //------------------------------------------------------------------------------
  /// Returns a list of unrecognized options from the command-line arguments
  //------------------------------------------------------------------------------
  pub fn parse_unrecognized(
    &self,
    recognized_options: &Vec<ParseOptionConfig>,
  ) -> Vec<String> {
    let mut unrecognized_set: HashSet<String> = HashSet::new();

    'outer: for arg in self.args.iter().skip(self.skip) {
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

impl ParseOptionConfig<'_> {
  fn make_hyphenated_option_name(
    &self,
    hyphenation_type: HyphenationType,
  ) -> Option<String> {
    match hyphenation_type {
      HyphenationType::Long => {
        let arg_option_name_long = self.name_long?;

        let hyphenated_option_name: String =
          format!("--{}", arg_option_name_long);

        Some(hyphenated_option_name)
      },
      HyphenationType::Short => {
        let arg_option_name_short = self.name_short?;

        let hyphenated_option_name: String =
          format!("-{}", arg_option_name_short);

        Some(hyphenated_option_name)
      },
    }
  }

  pub fn parse(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    if self.name_long.is_none() && self.name_short.is_none() {
      return ParseOutput {
        error: Some(CommanderParseError::ParseConfigNameless),
        index: None,
        value: None,
      };
    }

    for (arg_index, arg) in
      parse_input.args.iter().enumerate().skip(parse_input.skip)
    {
      let hyphenation_type_option: Option<HyphenationType> =
        HyphenationType::determine_hyphenation_type(arg);

      let Some(hyphenation_type) = hyphenation_type_option else {
        continue;
      };

      let hyphenated_option_name_option: Option<String> =
        self.make_hyphenated_option_name(hyphenation_type);

      let Some(hyphenated_option_name) = hyphenated_option_name_option else {
        continue;
      };

      let parse_output: ParseOutput =
        ParseOptionConfig::parse_hyphenated_option_name(
          arg,
          arg_index,
          &hyphenated_option_name,
          self.value_usage,
        );

      if parse_output.index.is_some() {
        return parse_output;
      }
    }

    ParseOutput::default()
  }

  fn parse_hyphenated_option_name(
    arg: &str,
    arg_index: usize,
    hyphenated_option_name: &str,
    value_usage: ValueUsage,
  ) -> ParseOutput {
    let hyphenated_option_name_equals: &String =
      &format!("{}=", hyphenated_option_name);

    let mut index_option: Option<usize> = None;

    let mut error_option: Option<CommanderParseError> = None;

    let mut value_option: Option<String> = None;

    if arg.starts_with(hyphenated_option_name_equals) {
      index_option = Some(arg_index);

      let value: &str =
        arg.strip_prefix(hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        error_option = Some(CommanderParseError::ValueMissingAfterEquals);
      } else {
        value_option = Some(value.to_string());

        if value_usage == ValueUsage::Verboten {
          error_option = Some(CommanderParseError::VerbotenValuePresent);
        }
      }
    } else if arg.eq(hyphenated_option_name) {
      index_option = Some(arg_index);

      if value_usage == ValueUsage::Required {
        error_option = Some(CommanderParseError::RequiredValueMissing);
      }
    }

    ParseOutput {
      error: error_option,
      index: index_option,
      value: value_option,
    }
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
