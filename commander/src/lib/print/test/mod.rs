//==============================================================================
//! Unit tests module Print module
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Created: 2022-04-02
//! - Updated: 2024-04-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

//-----------------------------------------------------------------------------
/// Unit test for make_print_option_prefix()
//-----------------------------------------------------------------------------
#[test]
fn test_make_print_option_prefix() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: Some("ARG_HELP_BRIEF_DESCRIPTION"),
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("ARG_HELP_NAME_LONG"),
    name_short: Some('T'),
    option_value: OptionValue::Prohibited,
  };
  let actual_prefix: String = make_print_option_prefix(&ARG_OPTION_TEST);
  assert_eq!("  -T, --ARG_HELP_NAME_LONG", actual_prefix);
}
