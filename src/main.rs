use clap::App;
use clap::Arg;

fn main() {
  let matches = App::new("CroftSoft Commander © 2022 CroftSoft Inc")
    .author("David Wallace Croft, david@croftsoft.com")
    .about("Command-line Rust example")
    .arg(
      Arg::new("name")
        .short('n')
        .long("name")
        .help("name to greet")
        .default_value("World"),
    )
    .get_matches();
  let name = matches.value_of("name").unwrap();
  println!("Hello, {}!", name);
}
