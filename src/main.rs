// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};

const APP_NAME: &str = "CroftSoft Commander © 2022 CroftSoft Inc";
const APP_ABOUT: &str = "Command-line Rust example";
const APP_AUTHOR: &str = "David Wallace Croft, david@croftsoft.com";
const ARG_NAME_HELP: &str = "name to greet";
const ARG_NAME_NAME: &str = "name";
const ARG_NAME_SHORT: char = 'n';
const ARG_NAME_TAKES_VALUE: bool = true;

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
  commander::greet(name_option);
}
