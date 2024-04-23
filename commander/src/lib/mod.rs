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
//! - Updated: 2024-04-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

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
/// Command-line option configuration
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct OptionConfig<'a> {
  pub brief_description: Option<&'a str>,
  pub default_value_bool: bool,
  pub is_type_bool: bool,
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
  pub value_usage: ValueUsage,
}

//------------------------------------------------------------------------------
/// Whether a option value is optional, prohibited, or required
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueUsage {
  Optional,
  Required,
  Verboten,
}

pub struct OptionConfig2<'a> {
  pub brief_description: Option<&'a str>,
  // TODO: Is there a static compile check to make at least one of these Some?
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
  pub value_usage: ValueUsage,
}
