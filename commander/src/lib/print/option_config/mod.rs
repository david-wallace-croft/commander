//==============================================================================
//! Module for OptionConfig
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-05
//! - Updated: 2024-06-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_input::ParseInput;
use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_output::ParseOutput;

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
}
