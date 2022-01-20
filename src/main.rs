// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};
use commander::*;

fn main() {
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
  let app: App = App::new(APP_NAME)
    .about(APP_ABOUT)
    .arg(interactive_arg)
    .arg(name_arg)
    .author(APP_AUTHOR);
  let arg_matches: ArgMatches = app.get_matches();
  let arg_match_interactive: Option<&str> =
    arg_matches.value_of(ARG_INTERACTIVE_NAME);
  let arg_match_name: Option<&str> = arg_matches.value_of(ARG_NAME_NAME);
  let interactive: bool = match arg_match_interactive {
    Some(interactive_string) => match interactive_string {
      "false" => false,
      _ => true,
    },
    None => true,
  };
  let name_option: Option<String> = match arg_match_name {
    Some(name) => Some(name.to_string()),
    None => None,
  };
  let main_args: MainArgs = MainArgs {
    interactive,
    name_option,
  };
  commander::main(main_args);
}
