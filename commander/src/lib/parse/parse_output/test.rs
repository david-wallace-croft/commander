//==============================================================================
//! Unit tests for module parse_output
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-29
//! - Updated: 2024-05-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;

#[test]
fn test_to_bool_result_0() {
  let test_parse_output = ParseOutput::default();

  let expected: Result<bool, CommanderParseError> = Ok(false);

  let actual: Result<bool, CommanderParseError> =
    test_parse_output.to_bool_result(false);

  assert_eq!(expected, actual);
}
