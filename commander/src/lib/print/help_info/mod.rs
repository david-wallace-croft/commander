//==============================================================================
//! Module for HelpInfo
//!
//! # Metadata
//! - Copyright: &copy; 2024-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-06-08
//! - Updated: 2025-11-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::app_info::AppInfo;
use super::option_config::OptionConfig;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Application and option data shown for the -\-help option
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct HelpInfo<'a> {
  pub app_info: &'a AppInfo<'a>,
  pub arg_options: &'a [OptionConfig<'a>],
}

impl HelpInfo<'_> {
  pub fn make_print_string(&self) -> String {
    let mut print_string: String = String::from("\n");

    print_string.push_str(&self.app_info.make_print_string());

    print_string.push_str("\nOPTIONS:\n");

    print_string
      .push_str(&OptionConfig::make_print_string_for_slice(self.arg_options));

    print_string
  }

  //----------------------------------------------------------------------------
  /// Prints the application and options descriptions
  //----------------------------------------------------------------------------
  pub fn print(&self) {
    print!("{}", self.make_print_string());
  }
}
