//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-27
//! - Updated: 2024-06-26
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
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

      if using_long_name {
        let arg_option_name: &str = arg.strip_prefix(prefix).unwrap();

        // TODO: Add a unit test for this
        if arg_option_name.eq("") {
          unrecognized_set.insert(String::from(""));

          continue;
        }

        for recognized_option in recognized_options {
          let name_long_option = recognized_option.name.get_name_long();

          if name_long_option.is_none() {
            continue;
          }

          let recognized_option_name: String =
            name_long_option.unwrap().to_string();

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

        continue;
      }

      let arg_option_names_with_equals: String =
        arg.strip_prefix(prefix).unwrap().to_string();

      let equals_index_option = arg_option_names_with_equals.find('=');

      let arg_option_names: String =
        if let Some(equals_index) = equals_index_option {
          arg_option_names_with_equals[0..equals_index].to_string()
        } else {
          arg_option_names_with_equals
        };

      // TODO: Add a unit test for this
      if arg_option_names.eq("") {
        unrecognized_set.insert(String::from(""));

        continue;
      }

      'middle: for arg_option_name in arg_option_names.chars() {
        for recognized_option in recognized_options {
          let name_short_option = recognized_option.name.get_name_short();

          if name_short_option.is_none() {
            continue;
          }

          let recognized_option_name: char = name_short_option.unwrap();

          if arg_option_name == recognized_option_name {
            continue 'middle;
          }
        }

        unrecognized_set.insert(String::from(arg_option_name));
      }
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
