//==============================================================================
//! Module for HelpInfo
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-06-08
//! - Updated: 2024-06-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::app_info::AppInfo;
use super::option_config::OptionConfig;
use super::print_options;

//------------------------------------------------------------------------------
/// Application and option data shown for the -\-help option
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct HelpInfo<'a> {
  pub app_info: &'a AppInfo<'a>,
  pub arg_options: &'a [OptionConfig<'a>],
}

impl HelpInfo<'_> {
  //----------------------------------------------------------------------------
  /// Prints the application and options descriptions
  //----------------------------------------------------------------------------
  pub fn print_help(&self) {
    println!();

    self.app_info.print_app_info();

    println!();

    println!("OPTIONS:");

    print_options(self.arg_options);
  }
}
