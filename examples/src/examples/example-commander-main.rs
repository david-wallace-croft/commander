//==============================================================================
//! CroftSoft Commander library usage example
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-08-07
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use std::env;

use commander::parse::parse_error::ParseError;
use commander::parse::parse_iterator::ParseIterator;
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
  let args: Vec<String> = env::args().collect();

  let arg_option_vector: Vec<&ParseOptionConfig> = OPTION_CONFIGS
    .iter()
    .map(|config| &config.parse_option_config)
    .collect();

  let help_wanted_parse_output_option: Option<ParseOutput> =
    OPTION_CONFIG_H.parse_last(&args);

  let help_wanted: bool =
    if let Some(help_wanted_parse_output) = help_wanted_parse_output_option {
      let help_wanted_result: Result<bool, ParseError> =
        help_wanted_parse_output.to_bool_result();

      // TODO: Show the user the parse error
      help_wanted_result.unwrap_or(false)
    } else {
      false
    };

  let interactive_parse_output_option: Option<ParseOutput> =
    OPTION_CONFIG_I.parse_last(&args);

  let interactive: Result<bool, ParseError> =
    if let Some(interactive_parse_output) = interactive_parse_output_option {
      interactive_parse_output.to_bool_result()
    } else {
      Ok(true)
    };

  // TODO: parse_option_type_string_with_default_value

  let name_parse_output_option: Option<ParseOutput> =
    OPTION_CONFIG_N.parse_last(&args);

  let name_option: Option<String> =
    if let Some(name_parse_output) = name_parse_output_option {
      name_parse_output.value
    } else {
      None
    };

  let quiet_parse_output_option: Option<ParseOutput> =
    OPTION_CONFIG_Q.parse_last(&args);

  // TODO: Show the user the parse error
  let quiet: bool = if let Some(quiet_parse_output) = quiet_parse_output_option
  {
    quiet_parse_output.to_bool_result().unwrap_or(false)
  } else {
    false
  };

  // TODO: Use a new(parse_option_configs) function
  let mut parse_iterator = ParseIterator {
    args: &args,
    parse_option_configs: &arg_option_vector,
    skip_arg: 1,
    skip_char: 0,
  };

  let unknown: Vec<ParseOutput> = parse_iterator.parse_unknown();

  OptionValues {
    help_wanted,
    interactive,
    name_option,
    quiet,
    unknown,
  }
}
