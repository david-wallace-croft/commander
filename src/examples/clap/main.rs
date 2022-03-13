mod app;

use app::clap_lib::args_from_clap;
use app::MainArgs;

fn main() {
  let main_args: MainArgs = args_from_clap();
  app::main(main_args);
}
