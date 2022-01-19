// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};
use commander::*;

fn main() {
  let name_arg = Arg::new(ARG_NAME_NAME)
    .help(ARG_NAME_HELP)
    .long(ARG_NAME_NAME)
    .short(ARG_NAME_SHORT)
    .takes_value(ARG_NAME_TAKES_VALUE);
  let app: App = App::new(APP_NAME)
    .about(APP_ABOUT)
    .arg(name_arg)
    .author(APP_AUTHOR);
  let arg_matches: ArgMatches = app.get_matches();
  let name_option: Option<&str> = arg_matches.value_of(ARG_NAME_NAME);
  commander::main(MainArgs { name_option });
}
