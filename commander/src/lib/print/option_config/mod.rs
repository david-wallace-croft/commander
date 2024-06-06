//==============================================================================
//! Module for OptionConfig
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-05
//! - Updated: 2024-06-06
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_input::ParseInput;
use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_output::ParseOutput;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Option configuration metadata for parsing and printing
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct OptionConfig<'a> {
  // TODO: Maybe move this to a PrintConfig
  pub brief_description: Option<&'a str>,
  pub parse_option_config: ParseOptionConfig<'a>,
}

impl OptionConfig<'_> {
  pub fn parse(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    self.parse_option_config.parse(parse_input)
  }

  //------------------------------------------------------------------------------
  /// String prefix for a command-line option shown for -\-help
  //------------------------------------------------------------------------------
  pub fn make_print_option_prefix(&self) -> String {
    let mut prefix: String = "".to_string();

    let parse_option_config: &ParseOptionConfig = &self.parse_option_config;

    let name_short: &Option<char> = &parse_option_config.name_short;

    let name_long = &parse_option_config.name_long;

    if name_short.is_some() {
      prefix.push_str("  -");

      prefix.push(name_short.unwrap());

      if name_long.is_some() {
        prefix.push_str(", --");

        prefix.push_str(name_long.unwrap());
      }
    } else {
      prefix.push_str("  --");

      prefix.push_str(name_long.unwrap());
    }

    prefix
  }
}
