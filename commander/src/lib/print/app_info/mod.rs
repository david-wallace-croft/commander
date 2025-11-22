//==============================================================================
//! Module for AppInfo
//!
//! # Metadata
//! - Copyright: &copy; 2024-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-07
//! - Updated: 2025-11-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

#[cfg(test)]
mod test;

//------------------------------------------------------------------------------
/// Application data shown for the -\-help option
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct AppInfo<'a> {
  pub about: Option<&'a str>,
  pub contact: Option<&'a str>,
  pub copyright: Option<&'a str>,
  pub name: Option<&'a str>,
}

impl AppInfo<'_> {
  pub fn make_print_string(&self) -> String {
    let mut app_info: String = String::new();

    if self.name.is_some() {
      app_info.push_str(self.name.unwrap());

      app_info.push('\n');
    }

    if self.copyright.is_some() {
      app_info.push_str(self.copyright.unwrap());

      app_info.push('\n');
    }

    if self.contact.is_some() {
      app_info.push_str(self.contact.unwrap());

      app_info.push('\n');
    }

    if self.about.is_some() {
      app_info.push_str(self.about.unwrap());

      app_info.push('\n');
    }

    app_info
  }

  //----------------------------------------------------------------------------
  /// Prints the application information
  //----------------------------------------------------------------------------
  pub fn print(&self) {
    print!("{}", self.make_print_string());
  }
}
