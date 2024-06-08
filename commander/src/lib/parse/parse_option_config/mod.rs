//==============================================================================
//! Module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-06-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::commander_parse_error::CommanderParseError;
use super::hyphenation_type::HyphenationType;
use super::parse_input::ParseInput;
use super::parse_output::ParseOutput;
use super::value_usage::ValueUsage;

#[cfg(test)]
mod test;

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

impl ParseOptionConfig<'_> {
  // TODO: rename parse() to parse_next(); then parse() -> Vec<ParseOutput>;

  //----------------------------------------------------------------------------
  /// Returns the next location of the option in the command-line arguments
  //----------------------------------------------------------------------------
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

  // private functions and methods

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
