//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-07-19
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use ::std::env;

use crate::parse::hyphenation_type::HyphenationType;
use crate::parse::parse_error::ParseError;
use crate::parse::parse_found::ParseFound;
use crate::parse::parse_output::ParseOutput;

use super::parse_option_config::ParseOptionConfig;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// The input to parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct ParseInput {
  // TODO: Can the args be immutable between calls to parse_next()?
  /// The command-line arguments
  pub args: Vec<String>,
  // TODO: Maybe make a ParseCursor struct
  /// How many command-line arguments to skip before searching for an option
  pub skip_arg: usize,
  /// How many chars within an argument to skip before searching for an option
  pub skip_char: usize,
}

impl ParseInput {
  //----------------------------------------------------------------------------
  /// A slice of the command-line arguments with skips of zero
  //----------------------------------------------------------------------------
  pub fn from_slice(args_slice: &[&str]) -> Self {
    let args: Vec<String> =
      args_slice.iter().map(|arg| arg.to_string()).collect();

    Self {
      args,
      skip_arg: 0,
      skip_char: 0,
    }
  }

  // TODO: more unit tests
  pub fn parse(
    &self,
    parse_option_configs: &[&ParseOptionConfig],
  ) -> Vec<ParseOutput> {
    let mut parse_output_vec = Vec::<ParseOutput>::new();

    let mut parse_output_option: Option<ParseOutput> =
      self.parse_next(parse_option_configs);

    loop {
      let Some(parse_output) = parse_output_option else {
        break;
      };

      let parse_input: ParseInput = match &parse_output.found {
        ParseFound::Long {
          arg_index,
          ..
        } => ParseInput {
          args: self.args.clone(),
          skip_arg: arg_index + 1,
          skip_char: 0,
        },
        ParseFound::Short {
          arg_index,
          char_index,
          ..
        } => ParseInput {
          args: self.args.clone(),
          skip_arg: *arg_index,
          skip_char: char_index + 1,
        },
      };

      parse_output_vec.push(parse_output);

      parse_output_option = parse_input.parse_next(parse_option_configs);
    }

    parse_output_vec
  }

  pub fn parse_next(
    &self,
    parse_option_configs: &[&ParseOptionConfig],
  ) -> Option<ParseOutput> {
    let mut skip_char = self.skip_char;

    for (arg_index, arg) in self.args.iter().enumerate().skip(self.skip_arg) {
      let hyphenation_type_option: Option<HyphenationType> =
        HyphenationType::determine_hyphenation_type(arg);

      let Some(hyphenation_type) = hyphenation_type_option else {
        continue;
      };

      match hyphenation_type {
        HyphenationType::Long => {
          return Some(self.parse_long(arg, arg_index, parse_option_configs));
        },
        HyphenationType::Short => {
          let parse_output_option =
            self.parse_short(arg, arg_index, parse_option_configs, skip_char);

          if parse_output_option.is_some() {
            return parse_output_option;
          }
        },
      };

      skip_char = 0;
    }

    None
  }

  //----------------------------------------------------------------------------
  /// Returns a list of unrecognized options from the command-line arguments
  //----------------------------------------------------------------------------
  // TODO: Replace parse_unrecognized() with filtered parse()
  // TODO: maybe rename to parse_unknown()
  pub fn parse_unrecognized(
    &self,
    recognized_options: &Vec<ParseOptionConfig>,
  ) -> Vec<ParseOutput> {
    // TODO: Maybe implement by filtering ParseInput.parse()
    let mut unrecognized_vec: Vec<ParseOutput> = Vec::new();

    for (arg_index, arg) in self.args.iter().enumerate().skip(self.skip_arg) {
      let (prefix, using_long_name) = if arg.starts_with("--") {
        ("--", true)
      } else if arg.starts_with('-') {
        ("-", false)
      } else {
        continue;
      };

      let arg_trimmed: &str = Self::trim_arg(arg, prefix);

      if arg_trimmed.is_empty() {
        // TODO: unit tests

        let name_long = if using_long_name {
          "".to_string()
        } else {
          "-".to_string()
        };

        let parse_output = ParseOutput {
          error: None,
          found: ParseFound::Long {
            arg_index,
            name_long,
          },
          known: None,
          value: None,
        };

        unrecognized_vec.push(parse_output);

        continue;
      }

      // TODO: parse the value

      if using_long_name {
        if !Self::matches_recognized_long(recognized_options, arg_trimmed) {
          let parse_output = ParseOutput {
            error: None,
            found: ParseFound::Long {
              arg_index,
              name_long: arg_trimmed.to_string(),
            },
            known: None,
            value: None,
          };

          unrecognized_vec.push(parse_output);
        }

        continue;
      }

      for option_name_short in arg_trimmed.chars() {
        if !Self::matches_recognized_short(
          recognized_options,
          option_name_short,
        ) {
          let parse_output = ParseOutput {
            error: None,
            found: ParseFound::Short {
              arg_index,
              // TODO: get actual char_index
              char_index: 0,
              name_short: option_name_short,
            },
            known: None,
            value: None,
          };

          unrecognized_vec.push(parse_output);
        }
      }
    }

    unrecognized_vec
  }

  // ---------------------------------------------------------------------------
  // private functions
  // ---------------------------------------------------------------------------

  fn matches_recognized_long(
    recognized_options: &Vec<ParseOptionConfig>,
    option_long_name: &str,
  ) -> bool {
    // TODO: Add a unit test for when arg_option_name is an empty string

    for recognized_option in recognized_options {
      let Some(recognized_name) = recognized_option.name.get_name_long() else {
        continue;
      };

      if option_long_name.eq(recognized_name) {
        return true;
      }
    }

    false
  }

  fn matches_recognized_short(
    recognized_options: &Vec<ParseOptionConfig>,
    option_short_name: char,
  ) -> bool {
    // TODO: Add a unit test for empty string

    for recognized_option in recognized_options {
      let Some(recognized_name) = recognized_option.name.get_name_short()
      else {
        continue;
      };

      if option_short_name == recognized_name {
        return true;
      }
    }

    false
  }

  fn parse_long(
    &self,
    arg: &str,
    arg_index: usize,
    parse_option_configs: &[&ParseOptionConfig],
  ) -> ParseOutput {
    for parse_option_config in parse_option_configs {
      if let Some(parse_output) = parse_option_config.parse_long(arg, arg_index)
      {
        return parse_output;
      }
    }

    let mut error: Option<ParseError> = None;

    let name_long_with_value: String = arg[2..].to_string();

    let mut name_long = name_long_with_value.clone();

    let mut value: Option<String> = None;

    let split_option: Option<(&str, &str)> =
      name_long_with_value.split_once('=');

    if let Some((split_name, split_value)) = split_option {
      name_long = split_name.to_string();

      if split_value.is_empty() {
        error = Some(ParseError::ValueMissingAfterEquals);
      } else {
        value = Some(split_value.to_string());
      }
    }

    ParseOutput {
      error,
      found: ParseFound::Long {
        arg_index,
        name_long,
      },
      known: None,
      value,
    }
  }

  pub(crate) fn parse_short(
    &self,
    arg: &str,
    arg_index: usize,
    parse_option_configs: &[&ParseOptionConfig],
    skip_char: usize,
  ) -> Option<ParseOutput> {
    let arg_without_prefix: &str = arg.strip_prefix('-').unwrap();

    let equals_index_option = arg_without_prefix.find('=');

    let mut arg_trimmed: &str = arg_without_prefix;

    let mut error: Option<ParseError> = None;

    let mut value_option: Option<&str> = if equals_index_option.is_none() {
      None
    } else {
      let equals_index: usize = equals_index_option.unwrap();

      arg_trimmed = &arg_without_prefix[0..equals_index];

      if skip_char != equals_index - 1 {
        None
      } else {
        let value_str: &str = &arg_without_prefix[equals_index + 1..];

        Some(value_str)
      }
    };

    let c_option: Option<char> = arg_trimmed.chars().nth(skip_char);

    let c = c_option?;

    for parse_option_config in parse_option_configs {
      let parse_output_option: Option<ParseOutput> = parse_option_config
        .parse_short_char(arg_index, c, skip_char, value_option);

      if parse_output_option.is_some() {
        return parse_output_option;
      }
    }

    if let Some(value_str) = value_option {
      if value_str.is_empty() {
        error = Some(ParseError::ValueMissingAfterEquals);

        value_option = None;
      }
    }

    let value: Option<String> =
      value_option.map(|value_str| value_str.to_string());

    Some(ParseOutput {
      error,
      found: ParseFound::Short {
        arg_index,
        char_index: skip_char,
        name_short: c,
      },
      known: None,
      value,
    })
  }

  //----------------------------------------------------------------------------
  /// Returns everything before the equals sign except for the prefix.
  /// Example: For a prefix of "--", "--abc=123" becomes "abc"
  //----------------------------------------------------------------------------
  fn trim_arg<'a>(
    arg: &'a str,
    prefix: &str,
  ) -> &'a str {
    let arg_stripped: &str = arg.strip_prefix(prefix).unwrap();

    let split_option: Option<(&str, &str)> = arg_stripped.split_once('=');

    match split_option {
      Some((before_equals, _)) => before_equals,
      None => arg_stripped,
    }
  }
}

impl Default for ParseInput {
  //----------------------------------------------------------------------------
  /// The command-line arguments with a skip_arg of one and skip_char of zero
  //----------------------------------------------------------------------------
  fn default() -> Self {
    Self {
      args: env::args().collect(),
      skip_arg: 1,
      skip_char: 0,
    }
  }
}
