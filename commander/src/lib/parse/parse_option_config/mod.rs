//==============================================================================
//! Module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-07-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::hyphenation_type::HyphenationType;
use super::parse_error::ParseError;
use super::parse_input::ParseInput;
use super::parse_option_name::ParseOptionName;
use super::parse_output::{ParseFound, ParseOutput};
use super::value_usage::ValueUsage;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Option configuration metadata for parsing
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct ParseOptionConfig<'a> {
  pub name: ParseOptionName<'a>,
  pub value_usage: ValueUsage,
}

impl ParseOptionConfig<'_> {
  // TODO: Move this to ParseInput so that it can return known=false;
  //   it would take in a slice of ParseOptionConfig as its input
  pub fn parse(
    &self,
    parse_input: &ParseInput,
  ) -> Vec<ParseOutput> {
    let mut parse_output_vec = Vec::<ParseOutput>::new();

    let mut parse_input_next: ParseInput = parse_input.clone();

    loop {
      let parse_output_option = self.parse_next(&parse_input_next);

      let Some(parse_output) = parse_output_option else {
        return parse_output_vec;
      };

      // TODO: Modify to iterate over chars in an arg with multiple short names
      // TODO: Use a parse_output.get_arg_index() function

      let skip: usize = match parse_output.found {
        ParseFound::Long {
          arg_index,
          ..
        } => arg_index,
        ParseFound::Short {
          arg_index,
          ..
        } => arg_index,
      } + 1;

      parse_input_next = ParseInput {
        args: parse_input.args.clone(),
        skip_arg: skip,
        skip_char: 0,
      };

      parse_output_vec.push(parse_output);
    }
  }

  pub fn parse_last(
    &self,
    parse_input: &ParseInput,
  ) -> Option<ParseOutput> {
    let mut parse_output_vec: Vec<ParseOutput> = self.parse(parse_input);

    parse_output_vec.pop()
  }

  //----------------------------------------------------------------------------
  /// Returns the next location of the option in the command-line arguments
  //----------------------------------------------------------------------------
  pub fn parse_next(
    &self,
    parse_input: &ParseInput,
  ) -> Option<ParseOutput> {
    for (arg_index, arg) in parse_input
      .args
      .iter()
      .enumerate()
      .skip(parse_input.skip_arg)
    {
      let hyphenation_type_option: Option<HyphenationType> =
        HyphenationType::determine_hyphenation_type(arg);

      let Some(hyphenation_type) = hyphenation_type_option else {
        continue;
      };

      // TODO: Update to use skip_char

      let parse_output_option: Option<ParseOutput> = match hyphenation_type {
        HyphenationType::Long => self.parse_long(arg, arg_index),
        HyphenationType::Short => self.parse_short(arg, arg_index),
      };

      if parse_output_option.is_some() {
        return parse_output_option;
      };
    }

    None
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

  fn parse_long(
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
      value: value_option,
    })
  }

  // TODO: Update README.md to show examples then update the standard output
  //   integration tests to test the examples
  fn parse_short(
    &self,
    arg: &str,
    arg_index: usize,
  ) -> Option<ParseOutput> {
    let name_short: char = self.name.get_name_short()?;

    let arg_without_prefix: &str = arg.strip_prefix('-').unwrap();

    let equals_index_option = arg_without_prefix.find('=');

    if equals_index_option.is_none() {
      let char_index: usize = arg_without_prefix.find(name_short)?;

      let found = ParseFound::Short {
        arg_index,
        char_index,
        name_short,
      };

      let error: Option<ParseError> =
        if self.value_usage == ValueUsage::Required {
          Some(ParseError::RequiredValueMissing)
        } else {
          None
        };

      return Some(ParseOutput {
        error,
        found,
        value: None,
      });
    }

    let equals_index: usize = equals_index_option.unwrap();

    let arg_prefix: &str = &arg_without_prefix[0..equals_index];

    let char_index = arg_prefix.find(name_short)?;

    let found = ParseFound::Short {
      arg_index,
      char_index,
      name_short,
    };

    if char_index != arg_prefix.len() - 1 {
      let error: Option<ParseError> =
        if self.value_usage == ValueUsage::Required {
          Some(ParseError::RequiredValueMissing)
        } else {
          None
        };

      return Some(ParseOutput {
        error,
        found,
        value: None,
      });
    }

    let value: &str = &arg_without_prefix[equals_index + 1..];

    if value.eq("") {
      return Some(ParseOutput {
        error: Some(ParseError::ValueMissingAfterEquals),
        found,
        value: None,
      });
    }

    let error: Option<ParseError> = if self.value_usage == ValueUsage::Verboten
    {
      Some(ParseError::VerbotenValuePresent)
    } else {
      None
    };

    Some(ParseOutput {
      error,
      found,
      value: Some(value.to_string()),
    })
  }
}
