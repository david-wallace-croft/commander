// https://docs.rs/clap/latest/clap/
use clap::{App, Arg, ArgMatches};
use std::io::{stdin, stdout, Error, Stdin, Write};

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
  let app: App = App::new("CroftSoft Commander © 2022 CroftSoft Inc")
    .author("David Wallace Croft, david@croftsoft.com")
    .about("Command-line Rust example")
    .arg(
      Arg::new("name")
        .long("name")
        .help("name to greet")
        .short('n')
        .takes_value(true),
    );
  let arg_matches: ArgMatches = app.get_matches();
  let name_option: Option<&str> = arg_matches.value_of("name");
  let name: String = match name_option {
    Some(arg_name) => arg_name.to_string(),
    None => ask(NAME_PROMPT, NAME_DEFAULT),
  };
  println!("Hello, {}!", name);
}
