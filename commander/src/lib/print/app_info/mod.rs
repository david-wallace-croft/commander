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

    if let Some(name) = self.name {
      app_info.push_str(name);

      app_info.push('\n');
    }

    if let Some(copyright) = self.copyright {
      app_info.push_str(copyright);

      app_info.push('\n');
    }

    if let Some(contact) = self.contact {
      app_info.push_str(contact);

      app_info.push('\n');
    }

    if let Some(about) = self.about {
      app_info.push_str(about);

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
