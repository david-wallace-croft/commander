//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-04-02
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

use crate::*;

//------------------------------------------------------------------------------
/// String prefix for a command-line option shown for -\-help
//------------------------------------------------------------------------------
pub fn make_print_option_prefix(arg_option: &OptionConfig) -> String {
  let mut prefix: String = "".to_string();
  if arg_option.name_short.is_some() {
    prefix.push_str("  -");
    prefix.push(arg_option.name_short.unwrap());
    if arg_option.name_long.is_some() {
      prefix.push_str(", --");
      prefix.push_str(arg_option.name_long.unwrap());
    }
  } else {
    prefix.push_str("  --");
    prefix.push_str(arg_option.name_long.unwrap());
  }
  prefix
}

//------------------------------------------------------------------------------
/// Prints the application description
//------------------------------------------------------------------------------
pub fn print_app_info(app_info: &AppInfo) {
  if app_info.name.is_some() {
    println!("{}", app_info.name.unwrap());
  }
  if app_info.copyright.is_some() {
    println!("{}", app_info.copyright.unwrap());
  }
  if app_info.contact.is_some() {
    println!("{}", app_info.contact.unwrap());
  }
  if app_info.about.is_some() {
    println!("{}", app_info.about.unwrap());
  }
}

//------------------------------------------------------------------------------
/// Prints the application and options descriptions
//------------------------------------------------------------------------------
pub fn print_help(help_info: &HelpInfo) {
  println!();
  print_app_info(help_info.app_info);
  println!();
  println!("OPTIONS:");
  print_options(help_info.arg_options);
}

//------------------------------------------------------------------------------
/// Prints a single option description
//------------------------------------------------------------------------------
pub fn print_option(
  arg_option: &OptionConfig,
  prefix_len_max: usize,
) {
  let mut line: String = "".to_string();
  let prefix: String = make_print_option_prefix(arg_option);
  line.push_str(&prefix);
  let spaces_count: usize = 2 + prefix_len_max - prefix.len();
  for _ in 0..spaces_count {
    line.push(' ');
  }
  if arg_option.brief_description.is_some() {
    line.push_str(arg_option.brief_description.unwrap());
  }
  println!("{}", line);
}

//------------------------------------------------------------------------------
/// Prints multiple option descriptions
//------------------------------------------------------------------------------
pub fn print_options(arg_options: &[OptionConfig]) {
  let mut prefix_len_max: usize = 0;
  for arg_option in arg_options {
    // TODO: save generated prefix
    let prefix: String = make_print_option_prefix(arg_option);
    let prefix_len: usize = prefix.len();
    if prefix_len > prefix_len_max {
      prefix_len_max = prefix_len;
    }
  }
  for arg_option in arg_options {
    print_option(arg_option, prefix_len_max);
  }
}
