use std::io::{stdin, stdout, Error, Stdin, Write};

const NAME_DEFAULT: &str = "World";
const NAME_PROMPT: &str = "What is your name? [World]: ";

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
        if trimmed_buffer.len() == 0 {
          return default.to_string();
        }
        return trimmed_buffer.to_string();
      }
      Err(error) => println!("ERROR: {}", error),
    }
  }
}

pub fn greet(name_option: Option<&str>) {
  let name: String = match name_option {
    Some(arg_name) => arg_name.to_string(),
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
