//==============================================================================
//! Module for ParseIterator
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-07-27
//! - Updated: 2024-07-30
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_found::ParseFound;
use crate::parse::parse_input::ParseInput;
use crate::parse::parse_option_config::ParseOptionConfig;
use crate::parse::parse_output::ParseOutput;

#[cfg(test)]
mod test;

pub struct ParseIterator<'a> {
  /// The command-line arguments
  // TODO: Maybe this should be Vec<String>
  pub args: &'a [String],
  /// The known command-line arguments options
  pub parse_option_configs: &'a [&'a ParseOptionConfig<'a>],
  /// How many command-line arguments to skip before searching for an option
  pub skip_arg: usize,
  /// How many chars within an argument to skip before searching for an option
  pub skip_char: usize,
}

impl<'a> Iterator for ParseIterator<'a> {
  type Item = ParseOutput;

  fn next(&mut self) -> Option<Self::Item> {
    let args: &Vec<String> =
      &self.args.iter().map(|arg| arg.to_string()).collect();

    let parse_input = ParseInput {
      args,
      skip_arg: self.skip_arg,
      skip_char: self.skip_char,
    };

    let parse_output_option: Option<ParseOutput> =
      parse_input.parse_next(self.parse_option_configs);

    let parse_output: ParseOutput = parse_output_option?;

    match &parse_output.found {
      ParseFound::Long {
        arg_index,
        ..
      } => {
        self.skip_arg = arg_index + 1;

        self.skip_char = 0;
      },
      ParseFound::Short {
        arg_index,
        char_index,
        ..
      } => {
        self.skip_arg = *arg_index;

        self.skip_char = char_index + 1;
      },
    };

    Some(parse_output)
  }
}
