use commander::{make_main_args, MainArgs};

fn main() {
  let main_args: MainArgs = make_main_args();
  commander::main(main_args);
}
