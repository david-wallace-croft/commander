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
  let main_options: MainOptions = MainOptions {
    help_wanted: false,
    interactive: false,
    name_option: None,
  };
  let actual_greeting = make_greeting(main_options);
  assert_eq!(actual_greeting, "Hello, World!");
}

#[test]
fn test_make_greeting_when_name_some() {
  let main_options: MainOptions = MainOptions {
    help_wanted: false,
    interactive: false,
    name_option: Some(String::from("Test")),
  };
  let actual_greeting = make_greeting(main_options);
  assert_eq!(actual_greeting, "Hello, Test!");
}
