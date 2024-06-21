//==============================================================================
//! Unit tests for module hyphenation_type
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-01
//! - Updated: 2024-06-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

#[test]
fn test_determine_hyphenation_type_0() {
  let test_arg: &str = "--TEST";

  let expected: Option<HyphenationType> = Some(HyphenationType::Long);

  let actual: Option<HyphenationType> =
    HyphenationType::determine_hyphenation_type(test_arg);

  assert_eq!(actual, expected);
}

#[test]
fn test_determine_hyphenation_type_1() {
  let test_arg: &str = "-T";

  let expected: Option<HyphenationType> = Some(HyphenationType::Short);

  let actual: Option<HyphenationType> =
    HyphenationType::determine_hyphenation_type(test_arg);

  assert_eq!(actual, expected);
}

#[test]
fn test_determine_hyphenation_type_2() {
  let test_arg: &str = "NONE";

  let expected: Option<HyphenationType> = None;

  let actual: Option<HyphenationType> =
    HyphenationType::determine_hyphenation_type(test_arg);

  assert_eq!(actual, expected);
}
