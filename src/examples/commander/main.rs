mod app;

use app::{make_main_args, MainArgs};

fn main() {
  let main_args: MainArgs = make_main_args();
  app::main(main_args);
}
