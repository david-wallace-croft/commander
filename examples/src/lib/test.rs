//==============================================================================
//! CroftSoft Commander library usage example unit tests
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-07-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

#[test]
fn test_make_greeting_when_name_none() {
  let option_values: OptionValues = OptionValues {
    help_wanted: false,
    interactive: Ok(false),
    name_option: None,
    quiet: false,
    unknown: Vec::new(),
  };

  let actual_greeting: String = make_greeting(option_values);

  assert_eq!(actual_greeting, "Hello, World!");
}

#[test]
fn test_make_greeting_when_name_some() {
  let option_values: OptionValues = OptionValues {
    help_wanted: false,
    interactive: Ok(false),
    name_option: Some(String::from("Test")),
    quiet: false,
    unknown: Vec::new(),
  };

  let actual_greeting: String = make_greeting(option_values);

  assert_eq!(actual_greeting, "Hello, Test!");
}
