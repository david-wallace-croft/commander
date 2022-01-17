// https://docs.rs/clap/latest/clap/
use clap::App;
use clap::Arg;

fn ask(prompt: &str) -> String {
  loop {
    let mut reply = String::new();
    println!("");
    println!("{}", prompt);
    let b1 = std::io::stdin().read_line(&mut reply).unwrap();
    if b1 != 0 {
      return reply.trim().to_string();
    }
  }
}

fn greet(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  let matches = App::new("CroftSoft Commander © 2022 CroftSoft Inc")
    .author("David Wallace Croft, david@croftsoft.com")
    .about("Command-line Rust example")
    .arg(
      Arg::new("name")
        .long("name")
        .help("name to greet")
        .short('n')
        .takes_value(true)
    )
    .get_matches();
  let name_option = matches.value_of("name");
  match name_option {
    Some(name) => greet(name),
    None => {
      let name = ask("What is your name?");
      greet(&name);
    },
  }
}
