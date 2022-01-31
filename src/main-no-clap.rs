mod args_lib;

use args_lib::*;
use commander::*;

fn main() {
  let main_args: MainArgs = make_main_args();
  commander::main(main_args);
}
