mod clap_lib;

use clap_lib::*;
use commander::*;

fn main() {
  let main_args: MainArgs = args_from_clap();
  commander::main(main_args);
}
