use std::io::{stdin, stdout, Error, Stdin, Write};

pub const APP_NAME: &str = "CroftSoft Commander © 2022 CroftSoft Inc";
pub const APP_ABOUT: &str = "Command-line Rust example";
pub const APP_AUTHOR: &str = "David Wallace Croft, david@croftsoft.com";
pub const ARG_NAME_HELP: &str = "name to greet";
pub const ARG_NAME_NAME: &str = "name";
pub const ARG_NAME_SHORT: char = 'n';
pub const ARG_NAME_TAKES_VALUE: bool = true;
pub const NAME_DEFAULT: &str = "World";
pub const NAME_PROMPT: &str = "What is your name? [World]: ";

pub fn ask(prompt: &str, default: &str) -> String {
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
        if trimmed_buffer.is_empty() {
          return default.to_string();
        }
        return trimmed_buffer.to_string();
      }
      Err(error) => println!("ERROR: {}", error),
    }
  }
}

#[derive(Debug)]
pub struct MainArgs {
  pub name_option: Option<String>,
}

pub fn main(main_args: MainArgs) {
  // println!("{:?}", main_args);
  // println!("{:#?}", main_args);
  let name: String = match main_args.name_option {
    Some(arg_name) => arg_name,
    None => ask(NAME_PROMPT, NAME_DEFAULT),
  };
  println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {

  // use super::*;

  #[test]
  fn test1() {
    assert_eq!(0, 0);
  }
}
