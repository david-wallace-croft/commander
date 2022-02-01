use commander::MainArgs;
use std::env;

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
