//==============================================================================
//! Module for ParseIterator.
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-08-04
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use crate::parse::hyphenation_type::HyphenationType;
use crate::parse::parse_error::ParseError;
use crate::parse::parse_found::ParseFound;
use crate::parse::parse_output::ParseOutput;
use crate::parse::value_usage::ValueUsage;

use super::parse_option_config::ParseOptionConfig;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// The input to parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseIterator<'a> {
  /// The command-line arguments
  pub args: &'a [String],
  /// The known command-line arguments options
  pub parse_option_configs: &'a [&'a ParseOptionConfig<'a>],
  /// How many command-line arguments to skip before searching for an option
  pub skip_arg: usize,
  /// How many chars within an argument to skip before searching for an option
  pub skip_char: usize,
}

impl<'a> ParseIterator<'a> {
  //----------------------------------------------------------------------------
  /// A slice of the command-line arguments with skips of zero
  //----------------------------------------------------------------------------
  pub fn from_slice(args: &'a [String]) -> Self {
    Self {
      args,
      skip_arg: 0,
      skip_char: 0,
      parse_option_configs: &[],
    }
  }

  pub fn parse(&mut self) -> Vec<ParseOutput> {
    self.collect()
  }

  //----------------------------------------------------------------------------
  /// Returns a list of unknown options from the command-line arguments
  //----------------------------------------------------------------------------
  pub fn parse_unknown(&mut self) -> Vec<ParseOutput> {
    self
      .parse()
      .into_iter()
      .filter(|parse_output| parse_output.known.is_none())
      .collect()
  }

  // ---------------------------------------------------------------------------
  // private functions
  // ---------------------------------------------------------------------------

  fn parse_long(
    &self,
    arg: &str,
    arg_index: usize,
  ) -> ParseOutput {
    for parse_option_config in self.parse_option_configs {
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

  fn parse_next(&self) -> Option<ParseOutput> {
    let mut skip_char = self.skip_char;

    for (arg_index, arg) in self.args.iter().enumerate().skip(self.skip_arg) {
      let hyphenation_type_option: Option<HyphenationType> =
        HyphenationType::determine_hyphenation_type(arg);

      let Some(hyphenation_type) = hyphenation_type_option else {
        continue;
      };

      match hyphenation_type {
        HyphenationType::Long => {
          return Some(self.parse_long(arg, arg_index));
        },
        HyphenationType::Short => {
          let parse_output_option = self.parse_short(arg, arg_index, skip_char);

          if parse_output_option.is_some() {
            return parse_output_option;
          }
        },
      };

      skip_char = 0;
    }

    None
  }

  fn parse_short(
    &self,
    arg: &str,
    arg_index: usize,
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

    for parse_option_config in self.parse_option_configs {
      let parse_output_option: Option<ParseOutput> = Self::parse_short_char(
        parse_option_config,
        arg_index,
        c,
        skip_char,
        value_option,
      );

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

  fn parse_short_char(
    parse_option_config: &ParseOptionConfig,
    arg_index: usize,
    c: char,
    char_index: usize,
    value_option: Option<&str>,
  ) -> Option<ParseOutput> {
    let name_short: char = parse_option_config.name.get_name_short()?;

    if c != name_short {
      return None;
    }

    let found = ParseFound::Short {
      arg_index,
      char_index,
      name_short,
    };

    let mut error: Option<ParseError> = None;

    let value: Option<String> = if let Some(value_str) = value_option {
      if value_str.is_empty() {
        error = Some(ParseError::ValueMissingAfterEquals);

        None
      } else {
        Some(value_str.to_string())
      }
    } else {
      None
    };

    if error.is_none() {
      error = match parse_option_config.value_usage {
        ValueUsage::Optional => None,
        ValueUsage::Required => {
          if value_option.is_none() {
            Some(ParseError::RequiredValueMissing)
          } else {
            None
          }
        },
        ValueUsage::Verboten => {
          if value_option.is_some() {
            Some(ParseError::VerbotenValuePresent)
          } else {
            None
          }
        },
      };
    }

    Some(ParseOutput {
      error,
      found,
      known: Some(parse_option_config.id.to_string()),
      value,
    })
  }
}

impl<'a> Iterator for ParseIterator<'a> {
  type Item = ParseOutput;

  fn next(&mut self) -> Option<Self::Item> {
    let parse_output_option: Option<ParseOutput> = self.parse_next();

    let parse_output: ParseOutput = parse_output_option?;

    match &parse_output.found {
      ParseFound::Long {
        arg_index,
        ..
      } => {
        self.skip_arg = arg_index + 1;

        self.skip_char = 0;
      },
      ParseFound::Short {
        arg_index,
        char_index,
        ..
      } => {
        self.skip_arg = *arg_index;

        self.skip_char = char_index + 1;
      },
    };

    Some(parse_output)
  }
}
