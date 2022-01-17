// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};
use std::io::{stdin, stdout, Error, Stdin, Write};

const APP_NAME: &str = "CroftSoft Commander © 2022 CroftSoft Inc";
const APP_ABOUT: &str = "Command-line Rust example";
const APP_AUTHOR: &str = "David Wallace Croft, david@croftsoft.com";
const ARG_NAME_HELP: &str = "name to greet";
const ARG_NAME_NAME: &str = "name";
const ARG_NAME_SHORT: char = 'n';
const ARG_NAME_TAKES_VALUE: bool = true;
const NAME_DEFAULT: &str = "World";
const NAME_PROMPT: &str = "What is your name? [World]: ";

fn ask(prompt: &str, default: &str) -> String {
  loop {
    println!("");
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut buffer: String = String::new();
    let stdin: Stdin = stdin();
    let result: Result<usize, Error> = stdin.read_line(&mut buffer);
    match result {
      Ok(_) => {
        let trimmed_buffer: &str = buffer.trim();
        if trimmed_buffer.len() == 0 {
          return default.to_string();
        }
        return trimmed_buffer.to_string();
      }
      Err(error) => println!("ERROR: {}", error),
    }
  }
}

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
  let name: String = match name_option {
    Some(arg_name) => arg_name.to_string(),
    None => ask(NAME_PROMPT, NAME_DEFAULT),
  };
  println!("Hello, {}!", name);
}
