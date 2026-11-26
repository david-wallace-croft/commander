//==============================================================================
//! Module for [`ParseFound`]
//!
//! # Metadata
//! - Copyright: &copy; 2024-2026 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-07-12
//! - Updated: 2026-11-26
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseFound {
  Long {
    arg_index: usize,
    name_long: String,
  },
  Short {
    arg_index: usize,
    char_index: usize,
    name_short: char,
  },
}

impl ParseFound {
  #[must_use]
  pub fn get_arg_index(&self) -> usize {
    match self {
      ParseFound::Long {
        arg_index,
        ..
      }
      | ParseFound::Short {
        arg_index,
        ..
      } => *arg_index,
    }
  }

  #[must_use]
  pub fn get_name(&self) -> String {
    match self {
      ParseFound::Long {
        name_long,
        ..
      } => name_long.clone(),
      ParseFound::Short {
        name_short,
        ..
      } => name_short.to_string(),
    }
  }
}
