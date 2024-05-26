//==============================================================================
//! Unit tests module Print module
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-26
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;
use crate::parse::ValueUsage;

//-----------------------------------------------------------------------------
/// Unit test for make_print_option_prefix()
//-----------------------------------------------------------------------------
#[test]
fn test_make_print_option_prefix() {
  const ARG_OPTION_TEST: OptionConfig = OptionConfig {
    brief_description: Some("ARG_HELP_BRIEF_DESCRIPTION"),
    parse_option_config: ParseOptionConfig {
      name_long: Some("ARG_HELP_NAME_LONG"),
      name_short: Some('T'),
      value_usage: ValueUsage::Verboten,
    },
  };
  let actual_prefix: String = make_print_option_prefix(&ARG_OPTION_TEST);
  assert_eq!("  -T, --ARG_HELP_NAME_LONG", actual_prefix);
}
