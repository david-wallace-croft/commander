use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, default_value = "World")]
  name: String,
  #[clap(short, long, default_value_t = 1)]
  count: u8,
}

fn main() {
  println!("Commander © 2022 CroftSoft Inc");
  let args = Args::parse();
  for _ in 0..args.count {
    println!("Hello, {}!", args.name);
  }
}
