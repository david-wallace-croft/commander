use crate::args_lib::{AppInfo, ArgOption, HelpInfo};

pub const APP_INFO_ABOUT: &str = "Command-line Rust example";
pub const APP_INFO_CONTACT: &str = "David Wallace Croft <david@CroftSoft.com>";
pub const APP_INFO_COPYRIGHT: &str = "Copyright © 2022 CroftSoft Inc";
pub const APP_INFO_NAME: &str = "CroftSoft Commander";
pub const ARG_INTERACTIVE_HELP: &str = "true/false, defaults to true";
pub const ARG_INTERACTIVE_NAME: &str = "interactive";
pub const ARG_INTERACTIVE_SHORT: char = 'i';
pub const ARG_INTERACTIVE_TAKES_VALUE: bool = true;
pub const ARG_NAME_HELP: &str = "Any value not starting with a hyphen (-)";
pub const ARG_NAME_NAME: &str = "name";
pub const ARG_NAME_SHORT: char = 'n';
pub const ARG_NAME_TAKES_VALUE: bool = true;
pub const NAME_DEFAULT: &str = "World";
pub const NAME_PROMPT: &str = "What is your name?";

pub const APP_INFO: AppInfo = AppInfo {
  about: Some(APP_INFO_ABOUT),
  contact: Some(APP_INFO_CONTACT),
  copyright: Some(APP_INFO_COPYRIGHT),
  name: Some(APP_INFO_NAME),
};

pub const ARG_OPTION_H: ArgOption = ArgOption {
  brief_description: Some("Show command-line options"),
  name_long: Some("help"),
  name_short: Some('h'),
};

pub const ARG_OPTION_I: ArgOption = ArgOption {
  brief_description: Some(ARG_INTERACTIVE_HELP),
  name_long: Some(ARG_INTERACTIVE_NAME),
  name_short: Some(ARG_INTERACTIVE_SHORT),
};

pub const ARG_OPTION_N: ArgOption = ArgOption {
  brief_description: Some(ARG_NAME_HELP),
  name_long: Some(ARG_NAME_NAME),
  name_short: Some(ARG_NAME_SHORT),
};

pub const ARG_OPTIONS: [ArgOption; 3] = [
  ARG_OPTION_H,
  ARG_OPTION_I,
  ARG_OPTION_N,
];

pub const HELP_INFO: HelpInfo = HelpInfo {
  app_info: &APP_INFO,
  arg_options: &ARG_OPTIONS,
};
