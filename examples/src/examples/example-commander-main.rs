//==============================================================================
//! CroftSoft Commander library usage example
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-06-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use commander::parse::parse_error::ParseError;
use commander::parse::parse_input::ParseInput;
use commander::parse::parse_option_config::ParseOptionConfig;
use commander::parse::parse_output::ParseOutput;
use croftsoft_commander_examples::constants::*;
use croftsoft_commander_examples::OptionValues;

//------------------------------------------------------------------------------
/// Parses the options using Commander and then runs the example application.
//------------------------------------------------------------------------------
fn main() {
  let option_values: OptionValues = parse_option_values_using_commander();

  croftsoft_commander_examples::main(option_values);
}

//------------------------------------------------------------------------------
/// Uses the CroftSoft Commander library to parse the application options
//------------------------------------------------------------------------------
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
pub fn parse_option_values_using_commander() -> OptionValues {
  let parse_input = &ParseInput::default();

  let help_wanted_result: Result<bool, ParseError> = OPTION_CONFIG_H
    .parse_last(parse_input)
    .to_bool_result(false);

  // TODO: Show the user the parse error
  let help_wanted: bool = help_wanted_result.unwrap_or(false);

  let interactive: Result<bool, ParseError> =
    OPTION_CONFIG_I.parse_last(parse_input).to_bool_result(true);

  // TODO: parse_option_type_string_with_default_value

  let parse_output: ParseOutput = OPTION_CONFIG_N.parse_last(parse_input);

  let name_option: Option<String> = parse_output.value;

  let arg_option_vector: Vec<ParseOptionConfig> = OPTION_CONFIGS
    .map(|config| config.parse_option_config)
    .to_vec();

  // TODO: Show the user the parse error
  let quiet = OPTION_CONFIG_Q
    .parse_next(parse_input)
    .to_bool_result(false)
    .unwrap_or(false);

  let unrecognized: Vec<String> =
    parse_input.parse_unrecognized(&arg_option_vector);

  OptionValues {
    help_wanted,
    interactive,
    name_option,
    quiet,
    unrecognized,
  }
}
