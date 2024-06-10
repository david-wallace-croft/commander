//==============================================================================
//! Functions to print application and option descriptions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-29
//! - Updated: 2024-06-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

pub mod app_info;
pub mod help_info;
pub mod option_config;

//------------------------------------------------------------------------------
/// Prints a message about an unrecognized option.
//------------------------------------------------------------------------------
pub fn print_unrecognized_option(unrecognized_option: &str) {
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
