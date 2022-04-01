//! An alternative command-line arguments parser library
//!
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Since: 2022-01-15
//!
//! [`CroftSoft Inc`]: http://www.croftsoft.com/
//! [`David Wallace Croft`]: http://www.croftsoft.com/people/david/

/// Application data shown for the -\-help option
#[derive(Debug)]
pub struct AppInfo<'a> {
  pub about: Option<&'a str>,
  pub contact: Option<&'a str>,
  pub copyright: Option<&'a str>,
  pub name: Option<&'a str>,
}

/// Command-line option configuration
#[derive(Debug)]
pub struct OptionDefinition<'a> {
  pub brief_description: Option<&'a str>,
  pub can_have_value: bool,
  pub default_value_bool: bool,
  pub is_type_bool: bool,
  pub name_short: Option<char>,
  pub name_long: Option<&'a str>,
}

// The boolean value for an option parsed from the command-line arguments
// #[derive(Debug)]
// pub struct OptionValueBool<'a> {
//   pub arg_option: OptionDefinition<'a>,
//   pub value: Option<bool>,
// }

/// Application and option data shown for the -\-help option
#[derive(Debug)]
pub struct HelpInfo<'a> {
  pub app_info: &'a AppInfo<'a>,
  pub arg_options: &'a [OptionDefinition<'a>],
}

/// Start of command-line option shown for -\-help
pub fn make_print_option_prefix(arg_option: &OptionDefinition) -> String {
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
  args_slice: &[String],
  arg_option: &OptionDefinition,
) -> bool {
  if arg_option.name_short.is_some() {
    let hyphenated_name_short = format!("-{}", arg_option.name_short.unwrap());
    if args_slice.contains(&hyphenated_name_short) {
      return true;
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long = format!("--{}", arg_option.name_long.unwrap());
    if args_slice.contains(&hyphenated_name_long) {
      return true;
    }
  }
  arg_option.default_value_bool
}

pub fn parse_option_type_bool_with_optional_value(
  args_slice: &[String],
  arg_option: &OptionDefinition,
) -> bool {
  let length: usize = args_slice.len();
  if arg_option.name_short.is_some() {
    let hyphenated_name_short: String =
      format!("-{}", arg_option.name_short.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_short) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        if value.eq("false") {
          return false;
        }
      }
      return true;
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long: String =
      format!("--{}", arg_option.name_long.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_long) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        if value.eq("false") {
          return false;
        }
      }
      return true;
    }
  }
  arg_option.default_value_bool
}

// TODO: Can we return a string slice instead of a String?
pub fn parse_option_type_string_with_required_value(
  args_slice: &[String],
  arg_option: &OptionDefinition,
) -> Option<String> {
  let length: usize = args_slice.len();
  if arg_option.name_short.is_some() {
    let hyphenated_name_short: String =
      format!("-{}", arg_option.name_short.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_short) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        // TODO: What if value starts with a hyphen?
        return Some(value.to_string());
      } else {
        return None;
      }
    }
  }
  if arg_option.name_long.is_some() {
    let hyphenated_name_long: String =
      format!("--{}", arg_option.name_long.unwrap());
    for index in 0..length {
      let arg: &String = &args_slice[index];
      if !arg.eq(&hyphenated_name_long) {
        continue;
      }
      if index < length - 1 {
        let value: &String = &args_slice[index + 1];
        // TODO: What if value starts with a hyphen?
        return Some(value.to_string());
      } else {
        return None;
      }
    }
  }
  None
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

pub fn print_option(arg_option: &OptionDefinition, prefix_len_max: usize) {
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

pub fn print_options(arg_options: &[OptionDefinition]) {
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

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_make_print_option_prefix() {
    const ARG_OPTION_TEST: OptionDefinition = OptionDefinition {
      brief_description: Some("ARG_HELP_BRIEF_DESCRIPTION"),
      can_have_value: false,
      default_value_bool: false,
      is_type_bool: true,
      name_long: Some("ARG_HELP_NAME_LONG"),
      name_short: Some('T'),
    };
    let actual_prefix = make_print_option_prefix(&ARG_OPTION_TEST);
    assert_eq!("  -T, --ARG_HELP_NAME_LONG", actual_prefix);
  }

  #[test]
  fn test_parse_option_type_bool_without_value() {
    const ARG_OPTION_TEST: OptionDefinition = OptionDefinition {
      brief_description: None,
      can_have_value: false,
      default_value_bool: false,
      is_type_bool: true,
      name_long: Some("TEST"),
      name_short: Some('T'),
    };
    let test_args_slice: &[String] = &["-T".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &["-t".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &["--TEST".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &["--test".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &["-TEST".to_string()];
    let actual_result =
      parse_option_type_bool_without_value(test_args_slice, &ARG_OPTION_TEST);
    assert_eq!(false, actual_result);
  }

  #[test]
  fn test_parse_option_type_bool_with_optional_value() {
    const ARG_OPTION_TEST: OptionDefinition = OptionDefinition {
      brief_description: None,
      can_have_value: false,
      default_value_bool: false,
      is_type_bool: true,
      name_long: Some("TEST"),
      name_short: Some('T'),
    };
    let test_args_slice: &[String] = &["-T".to_string()];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &[
      "-T".to_string(),
      "false".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &[
      "-T".to_string(),
      "true".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &["-t".to_string()];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &[
      "-t".to_string(),
      "true".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &["--TEST".to_string()];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &[
      "--TEST".to_string(),
      "false".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &[
      "--TEST".to_string(),
      "true".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(true, actual_result);
    let test_args_slice: &[String] = &["--test".to_string()];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &[
      "--test".to_string(),
      "true".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &["-TEST".to_string()];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
    let test_args_slice: &[String] = &[
      "-TEST".to_string(),
      "true".to_string(),
    ];
    let actual_result = parse_option_type_bool_with_optional_value(
      test_args_slice,
      &ARG_OPTION_TEST,
    );
    assert_eq!(false, actual_result);
  }
}
