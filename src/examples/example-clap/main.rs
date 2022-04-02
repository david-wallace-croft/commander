//==============================================================================
//! Command-Line Arguments Parser (clap) library usage example
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-01-15
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/
//==============================================================================

mod app;

use app::clap_lib::options_from_clap;
use app::MainOptions;

//------------------------------------------------------------------------------
/// Parses the options from the command-line and then runs the example
//------------------------------------------------------------------------------
fn main() {
  let main_options: MainOptions = options_from_clap();
  app::main(main_options);
}
