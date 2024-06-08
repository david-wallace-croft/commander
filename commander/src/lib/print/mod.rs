//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2024-06-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::print::option_config::OptionConfig;

pub mod app_info;
pub mod help_info;
pub mod option_config;

//------------------------------------------------------------------------------
/// Prints a single option description
//------------------------------------------------------------------------------
pub fn print_option(
  arg_option: &OptionConfig,
  prefix_len_max: usize,
) {
  let mut line: String = "".to_string();

  let prefix: String = arg_option.make_print_option_prefix();

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
    let prefix: String = arg_option.make_print_option_prefix();

    let prefix_len: usize = prefix.len();

    if prefix_len > prefix_len_max {
      prefix_len_max = prefix_len;
    }
  }

  for arg_option in arg_options {
    print_option(arg_option, prefix_len_max);
  }
}

//------------------------------------------------------------------------------
/// Prints a message about an unrecognized option.
//------------------------------------------------------------------------------
pub fn print_unrecognized_option(unrecognized_option: &String) {
  // TODO: Output to standard error?
  println!("Unrecognized option: \"{}\"", unrecognized_option);
}

//------------------------------------------------------------------------------
/// Prints a message about unrecognized options.
//------------------------------------------------------------------------------
pub fn print_unrecognized_options(unrecognized_options: &Vec<String>) {
  for unrecognized_option in unrecognized_options {
    print_unrecognized_option(unrecognized_option);
  }
}
