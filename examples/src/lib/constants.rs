//==============================================================================
//! CroftSoft Commander library usage example constants
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-07-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use commander::parse::parse_option_config::ParseOptionConfig;
use commander::parse::parse_option_name::ParseOptionName;
use commander::parse::value_usage::ValueUsage;
use commander::print::app_info::AppInfo;
use commander::print::help_info::HelpInfo;
use commander::print::option_config::OptionConfig;

pub const APP_INFO_ABOUT: &str = "Command-line arguments parser example";
pub const APP_INFO_CONTACT: &str = "David Wallace Croft <david@CroftSoft.com>";
pub const APP_INFO_COPYRIGHT: &str = "Copyright © 2022-2024 CroftSoft Inc";
pub const APP_INFO_NAME: &str = "CroftSoft Commander Example";

pub const ARG_HELP_HELP: &str = "Show command-line options";
pub const ARG_HELP_ID: &str = "help";
pub const ARG_HELP_NAME_LONG: &str = "help";
pub const ARG_HELP_NAME_SHORT: char = 'h';

pub const ARG_INTERACTIVE_HELP: &str = "true/false, defaults to true";
pub const ARG_INTERACTIVE_ID: &str = "interactive";
pub const ARG_INTERACTIVE_NAME_LONG: &str = "interactive";
pub const ARG_INTERACTIVE_NAME_SHORT: char = 'i';
// TODO: This one is clap-only.
pub const ARG_INTERACTIVE_TAKES_VALUE: bool = true;

pub const ARG_NAME_HELP: &str = "Any value not starting with a hyphen (-)";
pub const ARG_NAME_ID: &str = "name";
pub const ARG_NAME_NAME_LONG: &str = "name";
pub const ARG_NAME_NAME_SHORT: char = 'n';
// TODO: This one is clap-only.
pub const ARG_NAME_TAKES_VALUE: bool = true;

// TODO: Add this one to the clap example.
pub const ARG_QUIET_HELP: &str = "Suppress the exclamation mark";
pub const ARG_QUIET_ID: &str = "quiet";
pub const ARG_QUIET_NAME_LONG: &str = "quiet";
pub const ARG_QUIET_NAME_SHORT: char = 'q';

pub const NAME_DEFAULT: &str = "World";
pub const NAME_PROMPT: &str = "What is your name?";

pub const APP_INFO: AppInfo = AppInfo {
  about: Some(APP_INFO_ABOUT),
  contact: Some(APP_INFO_CONTACT),
  copyright: Some(APP_INFO_COPYRIGHT),
  name: Some(APP_INFO_NAME),
};

pub const OPTION_CONFIG_H: OptionConfig = OptionConfig {
  brief_description: Some(ARG_HELP_HELP),
  parse_option_config: ParseOptionConfig {
    id: ARG_HELP_ID,
    name: ParseOptionName::Both {
      name_long: ARG_HELP_NAME_LONG,
      name_short: ARG_HELP_NAME_SHORT,
    },
    value_usage: ValueUsage::Verboten,
  },
};

pub const OPTION_CONFIG_I: OptionConfig = OptionConfig {
  brief_description: Some(ARG_INTERACTIVE_HELP),
  parse_option_config: ParseOptionConfig {
    id: ARG_INTERACTIVE_ID,
    name: ParseOptionName::Both {
      name_long: ARG_INTERACTIVE_NAME_LONG,
      name_short: ARG_INTERACTIVE_NAME_SHORT,
    },
    value_usage: ValueUsage::Optional,
  },
};

pub const OPTION_CONFIG_N: OptionConfig = OptionConfig {
  brief_description: Some(ARG_NAME_HELP),
  parse_option_config: ParseOptionConfig {
    id: ARG_NAME_ID,
    name: ParseOptionName::Both {
      name_long: ARG_NAME_NAME_LONG,
      name_short: ARG_NAME_NAME_SHORT,
    },
    value_usage: ValueUsage::Required,
  },
};

pub const OPTION_CONFIG_Q: OptionConfig = OptionConfig {
  brief_description: Some(ARG_QUIET_HELP),
  parse_option_config: ParseOptionConfig {
    id: ARG_QUIET_ID,
    name: ParseOptionName::Both {
      name_long: ARG_QUIET_NAME_LONG,
      name_short: ARG_QUIET_NAME_SHORT,
    },
    value_usage: ValueUsage::Verboten,
  },
};

pub const OPTION_CONFIGS: [OptionConfig; 4] = [
  OPTION_CONFIG_H,
  OPTION_CONFIG_I,
  OPTION_CONFIG_N,
  OPTION_CONFIG_Q,
];

pub const HELP_INFO: HelpInfo = HelpInfo {
  app_info: &APP_INFO,
  arg_options: &OPTION_CONFIGS,
};
