//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-07-04
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use ::std::env;

use super::parse_option_config::ParseOptionConfig;

#[cfg(test)]
mod test;

// TODO: Move this to its own file
#[derive(Clone, Debug, PartialEq)]
pub struct ParseUnrecognizedOutput {
  // TODO: Maybe create an UnrecognizedOptionError and move to parse()
  // TODO: Maybe use this to replace ParseOutput with a boolean if recognized
  // TODO: add value after equals sign
  // TODO: add error for parsing value
  // TODO: include a sub-index for multiple short names in a single arg
  pub index: usize,
  pub name: String,
}

//------------------------------------------------------------------------------
/// The input to parsing an option from the command-line arguments
//------------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct ParseInput {
  /// The command-line arguments
  pub args: Vec<String>,
  /// How many command-line arguments to skip before searching for an option
  pub skip: usize,
}

impl ParseInput {
  //----------------------------------------------------------------------------
  /// A slice of the command-line arguments with a skip of zero
  //----------------------------------------------------------------------------
  pub fn from_slice(args_slice: &[&str]) -> Self {
    let args: Vec<String> =
      args_slice.iter().map(|arg| arg.to_string()).collect();

    Self {
      args,
      skip: 0,
    }
  }

  //----------------------------------------------------------------------------
  /// Returns a list of unrecognized options from the command-line arguments
  //----------------------------------------------------------------------------
  // TODO: maybe rename to parse_unknown()
  pub fn parse_unrecognized(
    &self,
    recognized_options: &Vec<ParseOptionConfig>,
  ) -> Vec<ParseUnrecognizedOutput> {
    let mut unrecognized_vec: Vec<ParseUnrecognizedOutput> = Vec::new();

    for (arg_index, arg) in self.args.iter().enumerate().skip(self.skip) {
      let (prefix, using_long_name) = if arg.starts_with("--") {
        ("--", true)
      } else if arg.starts_with('-') {
        ("-", false)
      } else {
        continue;
      };

      let arg_trimmed: &str = Self::trim_arg(arg, prefix);

      if using_long_name {
        if !Self::matches_recognized_long(recognized_options, arg_trimmed) {
          let parse_unrecognized_output = ParseUnrecognizedOutput {
            index: arg_index,
            name: arg_trimmed.to_string(),
          };

          unrecognized_vec.push(parse_unrecognized_output);
        }

        continue;
      }

      if arg_trimmed.is_empty() {
        let parse_unrecognized_output = ParseUnrecognizedOutput {
          index: arg_index,
          name: "".to_string(),
        };

        unrecognized_vec.push(parse_unrecognized_output);

        continue;
      }

      for option_name_short in arg_trimmed.chars() {
        if !Self::matches_recognized_short(
          recognized_options,
          option_name_short,
        ) {
          let parse_unrecognized_output = ParseUnrecognizedOutput {
            index: arg_index,
            // TODO: Should this be a char?
            name: option_name_short.to_string(),
          };

          unrecognized_vec.push(parse_unrecognized_output);
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
  /// The command-line arguments with a skip of one
  //----------------------------------------------------------------------------
  fn default() -> Self {
    Self {
      args: env::args().collect(),
      skip: 1,
    }
  }
}
