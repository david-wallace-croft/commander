//==============================================================================
//! CroftSoft Commander library usage example constants
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Created: 2022-01-15
//! - Updated: 2024-04-05
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use commander::{AppInfo, HelpInfo, OptionConfig};

pub const APP_INFO_ABOUT: &str = "Command-line arguments parser example";
pub const APP_INFO_CONTACT: &str = "David Wallace Croft <david@CroftSoft.com>";
pub const APP_INFO_COPYRIGHT: &str = "Copyright © 2022-2024 CroftSoft Inc";
pub const APP_INFO_NAME: &str = "CroftSoft Commander Example";
pub const ARG_HELP_BRIEF_DESCRIPTION: &str = "Show command-line options";
pub const ARG_HELP_NAME_LONG: &str = "help";
pub const ARG_HELP_NAME_SHORT: char = 'h';
pub const ARG_INTERACTIVE_HELP: &str = "true/false, defaults to true";
pub const ARG_INTERACTIVE_NAME: &str = "interactive";
pub const ARG_INTERACTIVE_SHORT: char = 'i';
// TODO: This one is clap-only.
pub const ARG_INTERACTIVE_TAKES_VALUE: bool = true;
pub const ARG_NAME_HELP: &str = "Any value not starting with a hyphen (-)";
pub const ARG_NAME_NAME: &str = "name";
pub const ARG_NAME_SHORT: char = 'n';
// TODO: This one is clap-only.
pub const ARG_NAME_TAKES_VALUE: bool = true;
pub const NAME_DEFAULT: &str = "World";
pub const NAME_PROMPT: &str = "What is your name?";

pub const APP_INFO: AppInfo = AppInfo {
  about: Some(APP_INFO_ABOUT),
  contact: Some(APP_INFO_CONTACT),
  copyright: Some(APP_INFO_COPYRIGHT),
  name: Some(APP_INFO_NAME),
};

pub const OPTION_CONFIG_H: OptionConfig = OptionConfig {
  brief_description: Some(ARG_HELP_BRIEF_DESCRIPTION),
  can_have_value: false,
  default_value_bool: false,
  is_type_bool: true,
  name_long: Some(ARG_HELP_NAME_LONG),
  name_short: Some(ARG_HELP_NAME_SHORT),
};

pub const OPTION_CONFIG_I: OptionConfig = OptionConfig {
  brief_description: Some(ARG_INTERACTIVE_HELP),
  can_have_value: true,
  default_value_bool: true,
  is_type_bool: true,
  name_long: Some(ARG_INTERACTIVE_NAME),
  name_short: Some(ARG_INTERACTIVE_SHORT),
};

pub const OPTION_CONFIG_N: OptionConfig = OptionConfig {
  brief_description: Some(ARG_NAME_HELP),
  can_have_value: true,
  default_value_bool: false,
  is_type_bool: false,
  name_long: Some(ARG_NAME_NAME),
  name_short: Some(ARG_NAME_SHORT),
};

pub const OPTION_CONFIGS: [OptionConfig; 3] = [
  OPTION_CONFIG_H,
  OPTION_CONFIG_I,
  OPTION_CONFIG_N,
];

pub const HELP_INFO: HelpInfo = HelpInfo {
  app_info: &APP_INFO,
  arg_options: &OPTION_CONFIGS,
};
