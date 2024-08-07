//==============================================================================
//! Module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-08-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_found::ParseFound;
use crate::parse::parse_input::ParseInput;

use super::hyphenation_type::HyphenationType;
use super::parse_error::ParseError;
use super::parse_iterator::ParseIterator;
use super::parse_option_name::ParseOptionName;
use super::parse_output::ParseOutput;
use super::value_usage::ValueUsage;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Option configuration metadata for parsing
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseOptionConfig<'a> {
  // TODO: Maybe make id any uniquely identifiable object
  pub id: &'a str,
  pub name: ParseOptionName<'a>,
  pub value_usage: ValueUsage,
}

impl<'a> ParseOptionConfig<'a> {
  pub fn parse(
    &self,
    args: &'a [String],
  ) -> Vec<ParseOutput> {
    let parse_input = ParseInput {
      args,
      parse_option_configs: &[self],
    };

    let parse_iterator: ParseIterator = parse_input.into_iter();

    parse_iterator
      .filter(|parse_output| parse_output.known.is_some())
      .collect()
  }

  pub fn parse_last(
    &self,
    args: &'a [String],
  ) -> Option<ParseOutput> {
    let mut parse_output_vec: Vec<ParseOutput> = self.parse(args);

    parse_output_vec.pop()
  }

  // ===========================================================================
  // private functions and methods
  // ===========================================================================

  fn make_hyphenated_option_name(
    &self,
    hyphenation_type: HyphenationType,
  ) -> Option<String> {
    match hyphenation_type {
      HyphenationType::Long => {
        let arg_option_name_long: &str = self.name.get_name_long()?;

        let hyphenated_option_name: String =
          format!("--{}", arg_option_name_long);

        Some(hyphenated_option_name)
      },
      HyphenationType::Short => {
        let arg_option_name_short: char = self.name.get_name_short()?;

        let hyphenated_option_name: String =
          format!("-{}", arg_option_name_short);

        Some(hyphenated_option_name)
      },
    }
  }

  // TODO: Can I move this to where it is used?
  pub(crate) fn parse_long(
    &self,
    arg: &str,
    arg_index: usize,
  ) -> Option<ParseOutput> {
    let hyphenated_option_name_option: Option<String> =
      self.make_hyphenated_option_name(HyphenationType::Long);

    let hyphenated_option_name = hyphenated_option_name_option?;

    let hyphenated_option_name_equals: &String =
      &format!("{}=", hyphenated_option_name);

    let mut found: bool = false;

    let mut error_option: Option<ParseError> = None;

    let mut value_option: Option<String> = None;

    if arg.starts_with(hyphenated_option_name_equals) {
      found = true;

      let value: &str =
        arg.strip_prefix(hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        error_option = Some(ParseError::ValueMissingAfterEquals);
      } else {
        value_option = Some(value.to_string());

        if self.value_usage == ValueUsage::Verboten {
          error_option = Some(ParseError::VerbotenValuePresent);
        }
      }
    } else if arg.eq(&hyphenated_option_name) {
      found = true;

      if self.value_usage == ValueUsage::Required {
        error_option = Some(ParseError::RequiredValueMissing);
      }
    }

    if !found {
      return None;
    }

    let parse_found: ParseFound = ParseFound::Long {
      arg_index,
      name_long: self.name.get_name_long().unwrap().to_string(),
    };

    Some(ParseOutput {
      error: error_option,
      found: parse_found,
      known: Some(self.id.to_string()),
      value: value_option,
    })
  }
}
