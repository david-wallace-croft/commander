//==============================================================================
//! Module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-08-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_found::ParseFound;

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

impl ParseOptionConfig<'_> {
  pub fn parse(
    &self,
    // TODO: Should this be mutable?
    parse_iterator: &ParseIterator,
  ) -> Vec<ParseOutput> {
    let mut inner_parse_iterator = ParseIterator {
      args: parse_iterator.args,
      parse_option_configs: &[self],
      skip_arg: parse_iterator.skip_arg,
      skip_char: parse_iterator.skip_char,
    };

    inner_parse_iterator
      .parse()
      .into_iter()
      .filter(|parse_output| parse_output.known.is_some())
      .collect()
  }

  pub fn parse_last(
    &self,
    parse_iterator: &ParseIterator,
  ) -> Option<ParseOutput> {
    let mut parse_output_vec: Vec<ParseOutput> = self.parse(parse_iterator);

    parse_output_vec.pop()
  }

  //----------------------------------------------------------------------------
  /// Returns the next location of the option in the command-line arguments
  //----------------------------------------------------------------------------
  pub fn parse_next(
    &self,
    parse_iterator: &ParseIterator,
  ) -> Option<ParseOutput> {
    // TODO: Make an iterator

    let mut skip_arg: usize = parse_iterator.skip_arg;

    let mut skip_char: usize = parse_iterator.skip_char;

    loop {
      let mut parse_iterator_next = ParseIterator {
        args: parse_iterator.args,
        parse_option_configs: &[self],
        skip_arg,
        skip_char,
      };

      let parse_output_option: Option<ParseOutput> = parse_iterator_next.next();

      let parse_output = parse_output_option?;

      if parse_output.known.is_some() {
        return Some(parse_output);
      }

      skip_arg = match parse_output.found {
        ParseFound::Long {
          arg_index,
          ..
        } => arg_index + 1,
        ParseFound::Short {
          arg_index,
          ..
        } => arg_index,
      };

      skip_char = match parse_output.found {
        ParseFound::Long {
          ..
        } => 0,
        ParseFound::Short {
          char_index,
          ..
        } => char_index + 1,
      };
    }
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
