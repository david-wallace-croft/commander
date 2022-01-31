use commander::{args_from_clap, MainArgs};

fn main() {
  let main_args: MainArgs = args_from_clap();
  commander::main(main_args);
}
