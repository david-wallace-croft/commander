//==============================================================================
//! Unit tests module Print module
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-04-02
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

//-----------------------------------------------------------------------------
/// Unit test for make_print_option_prefix()
//-----------------------------------------------------------------------------
#[test]
fn test_make_print_option_prefix() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: Some("ARG_HELP_BRIEF_DESCRIPTION"),
    can_have_value: false,
    default_value_bool: false,
    is_type_bool: true,
    name_long: Some("ARG_HELP_NAME_LONG"),
    name_short: Some('T'),
  };
  let actual_prefix: String = make_print_option_prefix(&ARG_OPTION_TEST);
  assert_eq!("  -T, --ARG_HELP_NAME_LONG", actual_prefix);
}
