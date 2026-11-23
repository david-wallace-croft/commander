//==============================================================================
//! Module parse_option_name
//!
//! # Metadata
//! - Copyright: &copy; 2024-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-19
//! - Updated: 2026-11-22
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseOptionName<'a> {
  Both {
    name_long: &'a str,
    name_short: char,
  },
  Long(&'a str),
  Short(char),
}

impl ParseOptionName<'_> {
  #[must_use]
  pub fn get_name_long(&self) -> Option<&'_ str> {
    match self {
      ParseOptionName::Both {
        name_long,
        ..
      }
      | ParseOptionName::Long(name_long) => Some(name_long),
      ParseOptionName::Short(_name_short) => None,
    }
  }

  #[must_use]
  pub fn get_name_short(&self) -> Option<char> {
    match self {
      ParseOptionName::Both {
        name_short,
        ..
      }
      | ParseOptionName::Short(name_short) => Some(*name_short),
      ParseOptionName::Long(_name_long) => None,
    }
  }
}
