//==============================================================================
//! Module for ParseInput.
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-08-04
//! - Updated: 2024-08-04
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use crate::parse::parse_iterator::ParseIterator;
use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_output::ParseOutput;

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseInput<'a> {
  /// The command-line arguments
  pub args: &'a [String],
  /// The known command-line arguments options
  pub parse_option_configs: &'a [&'a ParseOptionConfig<'a>],
}

impl<'a> IntoIterator for &ParseInput<'a> {
  type Item = ParseOutput;

  type IntoIter = ParseIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    ParseIterator {
      args: self.args,
      parse_option_configs: self.parse_option_configs,
      skip_arg: 0,
      skip_char: 0,
    }
  }
}
