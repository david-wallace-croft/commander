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
//! - Updated: 2024-06-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::print::option_config::OptionConfig;

use self::parse::parse_option_config::ParseOptionConfig;

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
