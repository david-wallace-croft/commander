//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-06-30
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
//==============================================================================

use ::std::collections::HashSet;
use ::std::env;

use super::parse_option_config::ParseOptionConfig;

#[cfg(test)]
mod test;

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
  pub fn parse_unrecognized(
    &self,
    recognized_options: &Vec<ParseOptionConfig>,
  ) -> Vec<String> {
    let mut unrecognized_set: HashSet<String> = HashSet::new();

    for arg in self.args.iter().skip(self.skip) {
      let (prefix, using_long_name) = if arg.starts_with("--") {
        ("--", true)
      } else if arg.starts_with('-') {
        ("-", false)
      } else {
        continue;
      };

      let arg_stripped: &str = arg.strip_prefix(prefix).unwrap();

      if using_long_name {
        if !Self::matches_recognized_long(recognized_options, arg_stripped) {
          unrecognized_set.insert(arg_stripped.to_string());
        }

        continue;
      }

      if !Self::matches_recognized_short(recognized_options, arg_stripped) {
        unrecognized_set.insert(arg_stripped.to_string());
      }
    }

    // TODO: Can this return a Set instead of an ordered Vec?

    Vec::from_iter(unrecognized_set)
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

      // TODO: Should the argument be stripped of equals before here?

      let recognized_name_with_equals: String = format!("{recognized_name}=");

      if option_long_name.starts_with(&recognized_name_with_equals) {
        return true;
      }
    }

    false
  }

  fn matches_recognized_short(
    recognized_options: &Vec<ParseOptionConfig>,
    option_short_names: &str,
  ) -> bool {
    // TODO: Should the argument be stripped of equals before here?

    let equals_index_option = option_short_names.find('=');

    let arg_option_names: &str = if let Some(equals_index) = equals_index_option
    {
      &option_short_names[0..equals_index]
    } else {
      option_short_names
    };

    // TODO: Add a unit test for empty string

    for recognized_option in recognized_options {
      let Some(recognized_name) = recognized_option.name.get_name_short()
      else {
        continue;
      };

      for arg_option_name in arg_option_names.chars() {
        if arg_option_name == recognized_name {
          return true;
        }
      }
    }

    false
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
