//==============================================================================
//! Module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-06-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::hyphenation_type::HyphenationType;
use super::parse_error::ParseError;
use super::parse_input::ParseInput;
use super::parse_option_name::ParseOptionName;
use super::parse_output::ParseOutput;
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
  pub fn parse(
    &self,
    parse_input: &ParseInput,
  ) -> Vec<ParseOutput> {
    let mut parse_output_vec = Vec::<ParseOutput>::new();

    let mut parse_input_next: ParseInput = parse_input.clone();

    loop {
      let parse_output = self.parse_next(&parse_input_next);

      if parse_output.index.is_none() {
        return parse_output_vec;
      }

      parse_input_next = ParseInput {
        args: parse_input.args.clone(),
        skip: parse_output.index.unwrap() + 1,
      };

      parse_output_vec.push(parse_output);
    }
  }

  pub fn parse_last(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    let mut parse_output_vec: Vec<ParseOutput> = self.parse(parse_input);

    let parse_output_option: Option<ParseOutput> = parse_output_vec.pop();

    if let Some(parse_output) = parse_output_option {
      return parse_output;
    }

    ParseOutput {
      error: None,
      index: None,
      value: None,
    }
  }

  //----------------------------------------------------------------------------
  /// Returns the next location of the option in the command-line arguments
  //----------------------------------------------------------------------------
  pub fn parse_next(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    let name_short_option = self.name.get_name_short();

    for (arg_index, arg) in
      parse_input.args.iter().enumerate().skip(parse_input.skip)
    {
      let hyphenation_type_option: Option<HyphenationType> =
        HyphenationType::determine_hyphenation_type(arg);

      let Some(hyphenation_type) = hyphenation_type_option else {
        continue;
      };

      if hyphenation_type == HyphenationType::Short {
        if name_short_option.is_none() {
          continue;
        }

        let parse_output: ParseOutput = self.parse_short(arg, arg_index);

        if parse_output.index.is_some() {
          return parse_output;
        }

        continue;
      }

      // TODO: Move everything that follows to a parse_long()
      //   and use a match statement on the hyphenation_type to
      //   determine which parse function to call

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

  fn parse_hyphenated_option_name(
    arg: &str,
    arg_index: usize,
    hyphenated_option_name: &str,
    value_usage: ValueUsage,
  ) -> ParseOutput {
    let hyphenated_option_name_equals: &String =
      &format!("{}=", hyphenated_option_name);

    let mut index_option: Option<usize> = None;

    let mut error_option: Option<ParseError> = None;

    let mut value_option: Option<String> = None;

    if arg.starts_with(hyphenated_option_name_equals) {
      index_option = Some(arg_index);

      let value: &str =
        arg.strip_prefix(hyphenated_option_name_equals).unwrap();

      if value.eq("") {
        error_option = Some(ParseError::ValueMissingAfterEquals);
      } else {
        value_option = Some(value.to_string());

        if value_usage == ValueUsage::Verboten {
          error_option = Some(ParseError::VerbotenValuePresent);
        }
      }
    } else if arg.eq(hyphenated_option_name) {
      index_option = Some(arg_index);

      if value_usage == ValueUsage::Required {
        error_option = Some(ParseError::RequiredValueMissing);
      }
    }

    ParseOutput {
      error: error_option,
      index: index_option,
      value: value_option,
    }
  }

  // TODO: Update README.md to show examples then update the standard output
  //   integration tests to test the examples
  fn parse_short(
    &self,
    arg: &str,
    arg_index: usize,
  ) -> ParseOutput {
    // TODO: What if there are multiple short names within one argument?
    //   Might need to return the sub_index in the ParseOutput.

    let name_short: char = self.name.get_name_short().unwrap();

    let equals_index_option = arg.find('=');

    if equals_index_option.is_none() {
      if arg.find(name_short).is_none() {
        return ParseOutput::default();
      };

      if self.value_usage == ValueUsage::Required {
        return ParseOutput {
          error: Some(ParseError::RequiredValueMissing),
          index: Some(arg_index),
          value: None,
        };
      }

      return ParseOutput {
        error: None,
        index: Some(arg_index),
        value: None,
      };
    }

    let equals_index: usize = equals_index_option.unwrap();

    let arg_prefix: &str = &arg[0..equals_index];

    let Some(sub_index) = arg_prefix.find(name_short) else {
      return ParseOutput::default();
    };

    if sub_index != arg_prefix.len() - 1 {
      if self.value_usage == ValueUsage::Required {
        return ParseOutput {
          error: Some(ParseError::RequiredValueMissing),
          index: Some(arg_index),
          value: None,
        };
      }

      return ParseOutput {
        error: None,
        index: Some(arg_index),
        value: None,
      };
    }

    let value: &str = &arg[equals_index + 1..];

    if value.eq("") {
      return ParseOutput {
        error: Some(ParseError::ValueMissingAfterEquals),
        index: Some(arg_index),
        value: None,
      };
    }

    if self.value_usage == ValueUsage::Verboten {
      return ParseOutput {
        error: Some(ParseError::VerbotenValuePresent),
        index: Some(arg_index),
        value: Some(value.to_string()),
      };
    }

    ParseOutput {
      error: None,
      index: Some(arg_index),
      value: Some(value.to_string()),
    }
  }
}
