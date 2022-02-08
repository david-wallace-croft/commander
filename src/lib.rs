mod args_lib;
pub mod constants;

use args_lib::*;
use constants::*;
use std::env;
use std::io::{stdin, stdout, Error, Stdin, Write};

#[derive(Debug)]
pub struct MainArgs {
  pub help_wanted: bool,
  pub interactive: bool,
  pub name_option: Option<String>,
}

pub fn ask(prompt: &str, default: &str) -> String {
  loop {
    println!();
    print!("{} [{}]: ", prompt, default);
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

pub fn main(main_args: MainArgs) {
  // println!("{:?}", main_args);
  // println!("{:#?}", main_args);
  if main_args.help_wanted {
    show_help();
    return;
  }
  let greeting: String = make_greeting(main_args);
  println!("{}", greeting);
}

// private functions

fn make_greeting(main_args: MainArgs) -> String {
  let name: String = match main_args.name_option {
    Some(arg_name) => {
      if main_args.interactive {
        ask(NAME_PROMPT, &arg_name)
      } else {
        arg_name
      }
    }
    None => {
      if main_args.interactive {
        ask(NAME_PROMPT, NAME_DEFAULT)
      } else {
        NAME_DEFAULT.to_string()
      }
    }
  };
  format!("Hello, {}!", name)
}

// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

pub fn make_main_args() -> MainArgs {
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);
  let length: usize = args.len();
  // println!("Args length = {}", length);
  let help_wanted: bool = args.contains(&String::from("--help"))
    || args.contains(&String::from("-h"));
  let mut interactive = true;
  for index in 2..length {
    let option: &String = &args[index - 1];
    let value: &String = &args[index];
    if (option.eq("-i") || option.eq("--interactive")) && value.eq("false") {
      interactive = false;
    }
  }
  let mut name_option = None;
  for index in 2..length {
    let option: &String = &args[index - 1];
    let value: &String = &args[index];
    if (option.eq("-n") || option.eq("--name")) && !value.starts_with('-') {
      name_option = Some(value.to_string());
      break;
    }
  }
  MainArgs {
    help_wanted,
    interactive,
    name_option,
  }
}

fn print_options(arg_options: Vec<ArgOption>) {
  for arg_option in arg_options {
    let mut line: String = "".to_string();
    if arg_option.name_short.is_some() {
      line.push_str("  -");
      line.push(arg_option.name_short.unwrap());
      if arg_option.name_long.is_some() {
        line.push_str(", --");
        line.push_str(arg_option.name_long.unwrap());
      }
    }
    if arg_option.brief_description.is_some() {
      line.push_str("        ");
      line.push_str(arg_option.brief_description.unwrap());
    }
    println!("{}", line);
  }
}

fn show_help() {
  println!("{}", APP_NAME);
  println!("{}", APP_AUTHOR);
  println!("{}", APP_ABOUT);
  println!();
  println!("OPTIONS:");
  // TODO: Pass in an array instead of a Vec
  print_options(Vec::from([ARG_OPTION_H]));
  print_options(Vec::from([ARG_OPTION_I]));
  print_options(Vec::from([ARG_OPTION_N]));
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_make_greeting_when_name_none() {
    let main_args: MainArgs = MainArgs {
      help_wanted: false,
      interactive: false,
      name_option: None,
    };
    let actual_greeting = make_greeting(main_args);
    assert_eq!(actual_greeting, "Hello, World!");
  }

  #[test]
  fn test_make_greeting_when_name_some() {
    let main_args: MainArgs = MainArgs {
      help_wanted: false,
      interactive: false,
      name_option: Some(String::from("Test")),
    };
    let actual_greeting = make_greeting(main_args);
    assert_eq!(actual_greeting, "Hello, Test!");
  }
}
