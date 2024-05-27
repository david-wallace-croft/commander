//==============================================================================
//! Modules to parse options from command-line arguments
//!
//! # Metadata
//! - Copyright: &copy; 2022-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-04-02
//! - Updated: 2024-05-27
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

pub mod commander_parse_error;
pub mod hyphenation_type;
pub mod parse_input;
pub mod parse_option_config;
pub mod parse_output;
pub mod value_usage;

#[cfg(test)]
mod test;
