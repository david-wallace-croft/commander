#[derive(Debug)]
pub struct AppInfo<'a> {
  pub about: Option<&'a str>,
  pub contact: Option<&'a str>,
  pub copyright: Option<&'a str>,
  pub name: Option<&'a str>,
}

// TODO: rename to OptionDefinition
#[derive(Debug)]
pub struct ArgOption<'a> {
  pub brief_description: Option<&'a str>,
  pub can_have_value: bool,
  pub default_value_bool: bool,
  pub is_type_bool: bool,
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
}

#[derive(Debug)]
pub struct OptionValueBool<'a> {
  pub arg_option: ArgOption<'a>,
  pub value: Option<bool>,
}

#[derive(Debug)]
pub struct HelpInfo<'a> {
  pub app_info: &'a AppInfo<'a>,
  pub arg_options: &'a [ArgOption<'a>],
}

pub fn make_print_option_prefix(arg_option: &ArgOption) -> String {
  let mut prefix: String = "".to_string();
  if arg_option.name_short.is_some() {
    prefix.push_str("  -");
    prefix.push(arg_option.name_short.unwrap());
    if arg_option.name_long.is_some() {
      prefix.push_str(", --");
      prefix.push_str(arg_option.name_long.unwrap());
    }
  } else {
    prefix.push_str("  --");
    prefix.push_str(arg_option.name_long.unwrap());
  }
  prefix
}

pub fn parse_option_type_bool_without_value(
  args: &[String],
  arg_option: &ArgOption) -> bool {
  if arg_option.name_short.is_some() {
    let hyphenated_name_short = format!("-{}", arg_option.name_short.unwrap());
    if args.contains(&hyphenated_name_short) {
      return true;
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long = format!("--{}", arg_option.name_long.unwrap());
    if args.contains(&hyphenated_name_long) {
      return true;
    }
  }
  arg_option.default_value_bool
}

pub fn print_app_info(app_info: &AppInfo) {
  if app_info.name.is_some() {
    println!("{}", app_info.name.unwrap());
  }
  if app_info.copyright.is_some() {
    println!("{}", app_info.copyright.unwrap());
  }
  if app_info.contact.is_some() {
    println!("{}", app_info.contact.unwrap());
  }
  if app_info.about.is_some() {
    println!("{}", app_info.about.unwrap());
  }
}

pub fn print_help(help_info: &HelpInfo) {
  println!();
  print_app_info(help_info.app_info);
  println!();
  println!("OPTIONS:");
  print_options(help_info.arg_options);
}

pub fn print_option(arg_option: &ArgOption, prefix_len_max: usize) {
  let mut line: String = "".to_string();
  let prefix = make_print_option_prefix(arg_option);
  line.push_str(&prefix);
  let spaces_count = 2 + prefix_len_max - prefix.len();
  for _ in 0..spaces_count {
    line.push(' ');
  }
  if arg_option.brief_description.is_some() {
    line.push_str(arg_option.brief_description.unwrap());
  }
  println!("{}", line);
}

pub fn print_options(arg_options: &[ArgOption]) {
  let mut prefix_len_max: usize = 0;
  for arg_option in arg_options {
    // TODO: save generated prefix
    let prefix = make_print_option_prefix(arg_option);
    let prefix_len = prefix.len();
    if prefix_len > prefix_len_max {
      prefix_len_max = prefix_len;
    }
  }
  for arg_option in arg_options {
    print_option(arg_option, prefix_len_max);
  }
}
