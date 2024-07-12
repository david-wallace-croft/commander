//==============================================================================
//! Command-Line Arguments Parser (clap) library usage example
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-07-11
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use clap::{Arg, ArgMatches, Command};

use commander::parse::parse_output::ParseOutput;
use croftsoft_commander_examples::constants::*;
use croftsoft_commander_examples::OptionValues;

//------------------------------------------------------------------------------
/// Parses the option values using clap and then runs the example application.
//------------------------------------------------------------------------------
fn main() {
  let option_values: OptionValues = parse_option_values_using_clap();

  croftsoft_commander_examples::main(option_values);
}

//------------------------------------------------------------------------------
/// Uses the clap library to parse the application options
//------------------------------------------------------------------------------
pub fn parse_option_values_using_clap() -> OptionValues {
  let interactive_arg: Arg = Arg::new(ARG_INTERACTIVE_NAME)
    .help(ARG_INTERACTIVE_HELP)
    .long(ARG_INTERACTIVE_NAME)
    .short(ARG_INTERACTIVE_SHORT)
    .takes_value(ARG_INTERACTIVE_TAKES_VALUE);

  let name_arg: Arg = Arg::new(ARG_NAME_NAME)
    .help(ARG_NAME_HELP)
    .long(ARG_NAME_NAME)
    .short(ARG_NAME_SHORT)
    .takes_value(ARG_NAME_TAKES_VALUE);

  let app_name: String = format!("{} {}", APP_INFO_NAME, APP_INFO_COPYRIGHT);

  let command: Command = Command::new(app_name)
    .about(APP_INFO_ABOUT)
    .arg(interactive_arg)
    .arg(name_arg)
    .author(APP_INFO_CONTACT);

  let arg_matches: ArgMatches = command.get_matches();

  let arg_match_interactive: Option<&str> =
    arg_matches.value_of(ARG_INTERACTIVE_NAME);

  let arg_match_name: Option<&str> = arg_matches.value_of(ARG_NAME_NAME);

  let interactive: bool = !matches!(arg_match_interactive, Some("false"));

  let name_option: Option<String> =
    arg_match_name.map(|name: &str| name.to_string());

  let unrecognized: Vec<ParseOutput> = Vec::new();

  OptionValues {
    help_wanted: false,
    interactive: Ok(interactive),
    name_option,
    // TODO: parse quiet
    quiet: false,
    unrecognized,
  }
}
