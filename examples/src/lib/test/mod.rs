//==============================================================================
//! CroftSoft Commander library usage example unit tests
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-01-15
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

#[test]
fn test_make_greeting_when_name_none() {
  let option_values: OptionValues = OptionValues {
    help_wanted: false,
    interactive: false,
    name_option: None,
    unrecognized: None,
  };
  let actual_greeting: String = make_greeting(option_values);
  assert_eq!(actual_greeting, "Hello, World!");
}

#[test]
fn test_make_greeting_when_name_some() {
  let option_values: OptionValues = OptionValues {
    help_wanted: false,
    interactive: false,
    name_option: Some(String::from("Test")),
    unrecognized: None,
  };
  let actual_greeting: String = make_greeting(option_values);
  assert_eq!(actual_greeting, "Hello, Test!");
}
