//==============================================================================
//! An alternative command-line arguments parser library
//!
//! # Usage
//! - See src/examples/commander/ for an example of how to use this libary
//! - Run the example using "cargo run -\-example commander -\- -\-help"
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-05-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use self::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_input::ParseInput;
use crate::parse::parse_output::ParseOutput;

pub mod parse;
pub mod print;

//------------------------------------------------------------------------------
/// Application data shown for the -\-help option
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct AppInfo<'a> {
  pub about: Option<&'a str>,
  pub contact: Option<&'a str>,
  pub copyright: Option<&'a str>,
  pub name: Option<&'a str>,
}

//------------------------------------------------------------------------------
/// Application and option data shown for the -\-help option
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct HelpInfo<'a> {
  pub app_info: &'a AppInfo<'a>,
  pub arg_options: &'a [OptionConfig<'a>],
}

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
