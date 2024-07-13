//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-07-13
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use ::std::env;

use crate::parse::parse_found::ParseFound;
use crate::parse::parse_output::ParseOutput;

use super::parse_option_config::ParseOptionConfig;

#[cfg(test)]
mod test;

// // TODO: Move this to its own file
// #[derive(Clone, Debug, PartialEq)]
// pub struct ParseUnrecognizedOutput {
//   // TODO: Maybe create an UnrecognizedOptionError and move to parse()
//   // TODO: Maybe use this to replace ParseOutput with a boolean if recognized
//   // TODO: add value after equals sign
//   // TODO: add error for parsing value
//   // TODO: include a sub-index for multiple short names in a single arg
//   pub index: usize,
//   pub name: String,
// }

//------------------------------------------------------------------------------
/// The input to parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct ParseInput {
  // TODO: Can the args be immutable between calls to parse_next()?
  /// The command-line arguments
  pub args: Vec<String>,
  // TODO: Maybe make a ParseCursor struct
  /// How many command-line arguments to skip before searching for an option
  pub skip_arg: usize,
  /// How many chars within an argument to skip before searching for an option
  pub skip_char: usize,
}

impl ParseInput {
  //----------------------------------------------------------------------------
  /// A slice of the command-line arguments with skips of zero
  //----------------------------------------------------------------------------
  pub fn from_slice(args_slice: &[&str]) -> Self {
    let args: Vec<String> =
      args_slice.iter().map(|arg| arg.to_string()).collect();

    Self {
      args,
      skip_arg: 0,
      skip_char: 0,
    }
  }

  // TODO: To be useful, needs to return the ParseOptionConfig matched
  // pub fn parse(
  //   &self,
  //   parse_option_configs: &[ParseOptionConfig],
  // ) -> Vec<ParseOutput> {
  //   let mut parse_output_vec = Vec::<ParseOutput>::new();
  //
  //   'outer: for (arg_index, arg) in self.args.iter().enumerate().skip(self.skip)
  //   {
  //     for parse_option_config in parse_option_configs {
  //       let parse_input = ParseInput {
  //         args: vec![arg.clone()],
  //         skip: 0,
  //       };
  //
  //       // TODO: Use something more efficient than parse_next
  //       let parse_output: ParseOutput =
  //         parse_option_config.parse_next(&parse_input);
  //
  //       if parse_output.index.is_some() {
  //         let new_parse_output = ParseOutput {
  //           error: parse_output.error,
  //           index: Some(arg_index),
  //           value: parse_output.value,
  //         };
  //
  //         parse_output_vec.push(new_parse_output);
  //
  //         continue 'outer;
  //       }
  //     }
  //
  //     // TODO: Attempt to parse unrecognized
  //   }
  //
  //   parse_output_vec
  // }

  //----------------------------------------------------------------------------
  /// Returns a list of unrecognized options from the command-line arguments
  //----------------------------------------------------------------------------
  // TODO: maybe rename to parse_unknown()
  pub fn parse_unrecognized(
    &self,
    recognized_options: &Vec<ParseOptionConfig>,
  ) -> Vec<ParseOutput> {
    let mut unrecognized_vec: Vec<ParseOutput> = Vec::new();

    for (arg_index, arg) in self.args.iter().enumerate().skip(self.skip_arg) {
      let (prefix, using_long_name) = if arg.starts_with("--") {
        ("--", true)
      } else if arg.starts_with('-') {
        ("-", false)
      } else {
        continue;
      };

      let arg_trimmed: &str = Self::trim_arg(arg, prefix);

      if arg_trimmed.is_empty() {
        // TODO: unit tests

        let name_long = if using_long_name {
          "".to_string()
        } else {
          "-".to_string()
        };

        let parse_output = ParseOutput {
          error: None,
          found: ParseFound::Long {
            arg_index,
            name_long,
          },
          known: None,
          value: None,
        };

        unrecognized_vec.push(parse_output);

        continue;
      }

      // TODO: parse the value

      if using_long_name {
        if !Self::matches_recognized_long(recognized_options, arg_trimmed) {
          let parse_output = ParseOutput {
            error: None,
            found: ParseFound::Long {
              arg_index,
              name_long: arg_trimmed.to_string(),
            },
            known: None,
            value: None,
          };

          unrecognized_vec.push(parse_output);
        }

        continue;
      }

      for option_name_short in arg_trimmed.chars() {
        if !Self::matches_recognized_short(
          recognized_options,
          option_name_short,
        ) {
          let parse_output = ParseOutput {
            error: None,
            found: ParseFound::Short {
              arg_index,
              // TODO: get actual char_index
              char_index: 0,
              name_short: option_name_short,
            },
            known: None,
            value: None,
          };

          unrecognized_vec.push(parse_output);
        }
      }
    }

    unrecognized_vec
  }

  // ---------------------------------------------------------------------------
  // private functions
  // ---------------------------------------------------------------------------

  fn matches_recognized_long(
    recognized_options: &Vec<ParseOptionConfig>,
    option_long_name: &str,
  ) -> bool {
    // TODO: Add a unit test for when arg_option_name is an empty string

    for recognized_option in recognized_options {
      let Some(recognized_name) = recognized_option.name.get_name_long() else {
        continue;
      };

      if option_long_name.eq(recognized_name) {
        return true;
      }
    }

    false
  }

  fn matches_recognized_short(
    recognized_options: &Vec<ParseOptionConfig>,
    option_short_name: char,
  ) -> bool {
    // TODO: Add a unit test for empty string

    for recognized_option in recognized_options {
      let Some(recognized_name) = recognized_option.name.get_name_short()
      else {
        continue;
      };

      if option_short_name == recognized_name {
        return true;
      }
    }

    false
  }

  //----------------------------------------------------------------------------
  /// Returns everything before the equals sign except for the prefix.
  /// Example: For a prefix of "--", "--abc=123" becomes "abc"
  //----------------------------------------------------------------------------
  fn trim_arg<'a>(
    arg: &'a str,
    prefix: &str,
  ) -> &'a str {
    let arg_stripped: &str = arg.strip_prefix(prefix).unwrap();

    let split_option: Option<(&str, &str)> = arg_stripped.split_once('=');

    match split_option {
      Some((before_equals, _)) => before_equals,
      None => arg_stripped,
    }
  }
}

impl Default for ParseInput {
  //----------------------------------------------------------------------------
  /// The command-line arguments with a skip_arg of one and skip_char of zero
  //----------------------------------------------------------------------------
  fn default() -> Self {
    Self {
      args: env::args().collect(),
      skip_arg: 1,
      skip_char: 0,
    }
  }
}
