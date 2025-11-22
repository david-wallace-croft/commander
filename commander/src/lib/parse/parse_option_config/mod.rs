//==============================================================================
//! Module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2025-11-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::parse_input::ParseInput;
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
  pub id: &'a str,
  pub name: ParseOptionName<'a>,
  pub value_usage: ValueUsage,
}

impl<'a> ParseOptionConfig<'a> {
  pub fn parse(
    &self,
    args: &'a [String],
  ) -> Vec<ParseOutput> {
    let parse_input: ParseInput<'_> = ParseInput {
      args,
      parse_option_configs: &[self],
    };

    let parse_iterator: ParseIterator = parse_input.into_iter();

    parse_iterator
      .filter(|parse_output: &ParseOutput| parse_output.known.is_some())
      .collect()
  }

  pub fn parse_last(
    &self,
    args: &'a [String],
  ) -> Option<ParseOutput> {
    let mut parse_output_vec: Vec<ParseOutput> = self.parse(args);

    parse_output_vec.pop()
  }
}
