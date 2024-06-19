//==============================================================================
//! Module parse_option_name
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-19
//! - Updated: 2024-06-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

// TODO: unit tests
// #[cfg(test)]
// mod test;

#[derive(Clone, Copy, Debug)]
pub enum ParseOptionName<'a> {
  Both {
    name_long: &'a str,
    name_short: char,
  },
  Long(&'a str),
  Short(char),
}

impl ParseOptionName<'_> {
  pub fn get_name_long(&self) -> Option<&'_ str> {
    match self {
      ParseOptionName::Both {
        name_long,
        ..
      } => Some(name_long),
      ParseOptionName::Long(name_long) => Some(name_long),
      ParseOptionName::Short(_name_short) => None,
    }
  }

  pub fn get_name_short(&self) -> Option<char> {
    match self {
      ParseOptionName::Both {
        name_short,
        ..
      } => Some(*name_short),
      ParseOptionName::Long(_name_long) => None,
      ParseOptionName::Short(name_short) => Some(*name_short),
    }
  }
}
