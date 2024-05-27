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

//------------------------------------------------------------------------------
/// Whether a option value is optional, required, or verboten (forbidden)
//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueUsage {
  Optional,
  Required,
  Verboten,
}
