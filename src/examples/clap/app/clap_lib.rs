// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};
use super::constants::*;
use super::MainArgs;

pub fn args_from_clap() -> MainArgs {
  let interactive_arg = Arg::new(ARG_INTERACTIVE_NAME)
    .help(ARG_INTERACTIVE_HELP)
    .long(ARG_INTERACTIVE_NAME)
    .short(ARG_INTERACTIVE_SHORT)
    .takes_value(ARG_INTERACTIVE_TAKES_VALUE);
  let name_arg = Arg::new(ARG_NAME_NAME)
    .help(ARG_NAME_HELP)
    .long(ARG_NAME_NAME)
    .short(ARG_NAME_SHORT)
    .takes_value(ARG_NAME_TAKES_VALUE);
  let app_name: String = format!("{} {}", APP_INFO_NAME, APP_INFO_COPYRIGHT);
  let app: App = App::new(app_name)
    .about(APP_INFO_ABOUT)
    .arg(interactive_arg)
    .arg(name_arg)
    .author(APP_INFO_CONTACT);
  let arg_matches: ArgMatches = app.get_matches();
  let arg_match_interactive: Option<&str> =
    arg_matches.value_of(ARG_INTERACTIVE_NAME);
  let arg_match_name: Option<&str> = arg_matches.value_of(ARG_NAME_NAME);
  let interactive: bool = !matches!(arg_match_interactive, Some("false"));
  let name_option: Option<String> = arg_match_name.map(|name| name.to_string());
  MainArgs {
    help_wanted: false,
    interactive,
    name_option,
  }
}
