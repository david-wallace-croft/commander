//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-05-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use ::std::collections::HashSet;
use ::std::env;

use super::parse_option_config::ParseOptionConfig;

//------------------------------------------------------------------------------
/// The input to parsing an option from the command-line arguments
//------------------------------------------------------------------------------
pub struct ParseInput {
  /// The command-line arguments
  pub args: Vec<String>,
  /// How many command-line arguments to skip before searching for an option
  pub skip: usize,
}

impl ParseInput {
  /// A slice of the command-line arguments with a skip of zero
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
  pub fn parse_unrecognized(
    &self,
    recognized_options: &Vec<ParseOptionConfig>,
  ) -> Vec<String> {
    let mut unrecognized_set: HashSet<String> = HashSet::new();

    'outer: for arg in self.args.iter().skip(self.skip) {
      let (prefix, using_long_name) = if arg.starts_with("--") {
        ("--", true)
      } else if arg.starts_with('-') {
        ("-", false)
      } else {
        continue;
      };

      let arg_option_name: &str = arg.strip_prefix(prefix).unwrap();

      if arg_option_name.eq("") {
        unrecognized_set.insert(String::from(""));

        continue;
      }

      for recognized_option in recognized_options {
        let recognized_option_name: String = if using_long_name {
          if recognized_option.name_long.is_none() {
            continue;
          }

          recognized_option.name_long.unwrap().to_string()
        } else {
          if recognized_option.name_short.is_none() {
            continue;
          }

          recognized_option.name_short.unwrap().to_string()
        };

        if arg_option_name.eq(&recognized_option_name) {
          continue 'outer;
        }

        let recognized_option_name_equals: String =
          format!("{recognized_option_name}=");

        if arg_option_name.starts_with(&recognized_option_name_equals) {
          continue 'outer;
        }
      }

      unrecognized_set.insert(String::from(arg_option_name));
    }

    Vec::from_iter(unrecognized_set)
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
