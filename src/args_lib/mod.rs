#[derive(Debug)]
pub struct ArgOption<'a> {
  pub brief_description: Option<&'a str>,
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
}
