//==============================================================================
//! Unit tests for HelpInfo
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-08
//! - Updated: 2024-06-08
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::value_usage::ValueUsage;

use super::*;

const TEST_APP_INFO_0: AppInfo = AppInfo {
  about: Some("TEST_ABOUT_0"),
  contact: Some("TEST_CONTACT_0"),
  copyright: Some("TEST_COPYRIGHT_0"),
  name: Some("TEST_NAME_0"),
};

const ARG_OPTION_TEST_0: OptionConfig = OptionConfig {
  brief_description: Some("TEST_BRIEF_DESCRIPTION_0"),
  parse_option_config: ParseOptionConfig {
    name_long: Some("TEST_NAME_LONG_0"),
    name_short: Some('0'),
    value_usage: ValueUsage::Required,
  },
};

const ARG_OPTION_TEST_1: OptionConfig = OptionConfig {
  brief_description: Some("TEST_BRIEF_DESCRIPTION_1"),
  parse_option_config: ParseOptionConfig {
    name_long: Some("TEST_NAME_LONG_1"),
    name_short: Some('1'),
    value_usage: ValueUsage::Verboten,
  },
};

#[test]
fn test_make_print_string_0() {
  let test_help_info = HelpInfo {
    app_info: &TEST_APP_INFO_0,
    arg_options: &[
      ARG_OPTION_TEST_0,
      ARG_OPTION_TEST_1,
    ],
  };

  let expected: String = String::from(
    r#"
TEST_NAME_0
TEST_COPYRIGHT_0
TEST_CONTACT_0
TEST_ABOUT_0

OPTIONS:
  -0, --TEST_NAME_LONG_0  TEST_BRIEF_DESCRIPTION_0
  -1, --TEST_NAME_LONG_1  TEST_BRIEF_DESCRIPTION_1
"#,
  );

  let actual: String = test_help_info.make_print_string();

  assert_eq!(expected, actual);
}
