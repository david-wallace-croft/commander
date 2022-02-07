mod clap_lib;

use clap_lib::args_from_clap;
use commander::MainArgs;

fn main() {
  let main_args: MainArgs = args_from_clap();
  commander::main(main_args);
}
