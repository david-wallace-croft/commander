//==============================================================================
//! Unit tests for AppInfo
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-07
//! - Updated: 2024-06-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

const TEST_ABOUT: &str = "TEST_ABOUT";

const TEST_CONTACT: &str = "TEST_CONTACT";

const TEST_COPYRIGHT: &str = "TEST_COPYRIGHT";

const TEST_NAME: &str = "TEST_NAME";

const TEST_APP_INFO: AppInfo = AppInfo {
  about: Some(TEST_ABOUT),
  contact: Some(TEST_CONTACT),
  copyright: Some(TEST_COPYRIGHT),
  name: Some(TEST_NAME),
};

#[test]
fn test_make_print_string_0() {
  let expected: String = format!(
    "{}\n{}\n{}\n{}\n",
    TEST_NAME, TEST_COPYRIGHT, TEST_CONTACT, TEST_ABOUT,
  );

  let actual: String = TEST_APP_INFO.make_print_string();

  assert_eq!(expected, actual);
}
