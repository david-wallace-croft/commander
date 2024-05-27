//==============================================================================
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-01-15
//! - Updated: 2024-05-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

pub enum HyphenationType {
  Long,
  Short,
}

impl HyphenationType {
  pub fn determine_hyphenation_type(arg: &str) -> Option<Self> {
    if arg.starts_with("--") {
      Some(Self::Long)
    } else if arg.starts_with('-') {
      Some(Self::Short)
    } else {
      None
    }
  }
}
