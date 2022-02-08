use crate::args_lib::ArgOption;

pub const APP_NAME: &str = "CroftSoft Commander © 2022 CroftSoft Inc";
pub const APP_ABOUT: &str = "Command-line Rust example";
pub const APP_AUTHOR: &str = "David Wallace Croft <david@croftsoft.com>";
pub const ARG_INTERACTIVE_HELP: &str = "prompt user for inputs";
pub const ARG_INTERACTIVE_NAME: &str = "interactive";
pub const ARG_INTERACTIVE_SHORT: char = 'i';
pub const ARG_INTERACTIVE_TAKES_VALUE: bool = true;
pub const ARG_NAME_HELP: &str = "name to greet";
pub const ARG_NAME_NAME: &str = "name";
pub const ARG_NAME_SHORT: char = 'n';
pub const ARG_NAME_TAKES_VALUE: bool = true;
pub const NAME_DEFAULT: &str = "World";
pub const NAME_PROMPT: &str = "What is your name?";

pub const ARG_OPTION_H: ArgOption = ArgOption {
  brief_description: Some("Show command-line options"),
  name_long: Some("help"),
  name_short: Some('h'),
};

pub const ARG_OPTION_I: ArgOption = ArgOption {
  brief_description: Some("true/false, defaults to true"),
  name_long: Some("interactive"),
  name_short: Some('i'),
};

pub const ARG_OPTION_N: ArgOption = ArgOption {
  brief_description: Some("Any value not starting with a hyphen (-)"),
  name_long: Some("name"),
  name_short: Some('n'),
};
