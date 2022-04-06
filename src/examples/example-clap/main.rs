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

use app::clap_lib::parse_option_values_using_clap;
use app::OptionValues;

//------------------------------------------------------------------------------
/// Parses the option values using clap and then runs the example application
//------------------------------------------------------------------------------
fn main() {
  let option_values: OptionValues = parse_option_values_using_clap();
  app::main(option_values);
}
