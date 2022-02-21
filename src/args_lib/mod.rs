#[derive(Debug)]
pub struct AppInfo<'a> {
  pub about: Option<&'a str>,
  pub contact: Option<&'a str>,
  pub copyright: Option<&'a str>,
  pub name: Option<&'a str>,
}

#[derive(Debug)]
pub struct ArgOption<'a> {
  pub brief_description: Option<&'a str>,
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
}
