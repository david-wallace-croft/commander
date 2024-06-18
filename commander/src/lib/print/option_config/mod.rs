//==============================================================================
//! Module for OptionConfig
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-05
//! - Updated: 2024-06-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_input::ParseInput;
use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_output::ParseOutput;

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Option configuration metadata for parsing and printing
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct OptionConfig<'a> {
  // TODO: Maybe move this to a PrintConfig
  pub brief_description: Option<&'a str>,
  pub parse_option_config: ParseOptionConfig<'a>,
}

impl OptionConfig<'_> {
  //------------------------------------------------------------------------------
  /// String prefix for a command-line option shown for -\-help
  //------------------------------------------------------------------------------
  pub fn make_print_option_prefix(&self) -> String {
    let mut prefix: String = "".to_string();

    let parse_option_config: &ParseOptionConfig = &self.parse_option_config;

    let name_short: &Option<char> = &parse_option_config.name_short;

    let name_long = &parse_option_config.name_long;

    if name_short.is_some() {
      prefix.push_str("  -");

      prefix.push(name_short.unwrap());

      if name_long.is_some() {
        prefix.push_str(", --");

        prefix.push_str(name_long.unwrap());
      }
    } else {
      prefix.push_str("  --");

      prefix.push_str(name_long.unwrap());
    }

    prefix
  }

  pub fn make_print_string(
    &self,
    prefix_len_max: usize,
  ) -> String {
    let mut line: String = "".to_string();

    let prefix: String = self.make_print_option_prefix();

    line.push_str(&prefix);

    let spaces_count: usize = 2 + prefix_len_max - prefix.len();

    for _ in 0..spaces_count {
      line.push(' ');
    }

    if self.brief_description.is_some() {
      line.push_str(self.brief_description.unwrap());
    }

    line
  }

  pub fn make_print_string_for_slice(arg_options: &[OptionConfig]) -> String {
    let mut prefix_len_max: usize = 0;

    for arg_option in arg_options {
      // TODO: save generated prefix
      let prefix: String = arg_option.make_print_option_prefix();

      let prefix_len: usize = prefix.len();

      if prefix_len > prefix_len_max {
        prefix_len_max = prefix_len;
      }
    }

    let mut print_string = String::new();

    for arg_option in arg_options {
      print_string.push_str(&arg_option.make_print_string(prefix_len_max));

      print_string.push('\n');
    }

    print_string
  }

  // TODO: unit tests
  pub fn parse(
    &self,
    parse_input: &ParseInput,
  ) -> Vec<ParseOutput> {
    self.parse_option_config.parse(parse_input)
  }

  pub fn parse_last(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    self.parse_option_config.parse_last(parse_input)
  }

  pub fn parse_next(
    &self,
    parse_input: &ParseInput,
  ) -> ParseOutput {
    self.parse_option_config.parse_next(parse_input)
  }

  //----------------------------------------------------------------------------
  /// Prints a single option description
  //----------------------------------------------------------------------------
  pub fn print_option(
    &self,
    prefix_len_max: usize,
  ) {
    println!("{}", self.make_print_string(prefix_len_max));
  }

  //----------------------------------------------------------------------------
  /// Prints multiple option descriptions
  //----------------------------------------------------------------------------
  pub fn print_options(arg_options: &[OptionConfig]) {
    print!("{}", Self::make_print_string_for_slice(arg_options));
  }
}
