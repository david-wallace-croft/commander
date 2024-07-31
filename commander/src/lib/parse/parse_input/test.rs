//==============================================================================
//! Unit tests for module parse_input
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-31
//! - Updated: 2024-07-31
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use std::string::ToString;

use crate::parse::parse_error::ParseError;
use crate::parse::parse_option_name::ParseOptionName;
use crate::parse::value_usage::ValueUsage;

use super::*;

const TEST_ID_0: &str = "TEST_ID_0";

const TEST_ID_1: &str = "TEST_ID_1";

const TEST_PARSE_OPTION_CONFIG_0: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_0,
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIG_1: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_1,
  name: ParseOptionName::Long(""),
  value_usage: ValueUsage::Verboten,
};

#[test]
fn test_from_slice_0() {
  let test_args_slice: Vec<String> = vec!["TEST".to_string()];

  let args = &vec!["TEST".to_string()];

  let expected: ParseInput = ParseInput {
    args,
    skip_arg: 0,
    skip_char: 0,
  };

  let actual: ParseInput = ParseInput::from_slice(&test_args_slice);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_0() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "-T".to_string(),
  ];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let test_parse_option_configs: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Long {
        arg_index: 0,
        name_long: "TEST".to_string(),
      },
      known: Some(TEST_ID_0.to_string()),
      value: None,
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 1,
        char_index: 0,
        name_short: 'T',
      },
      known: Some(TEST_ID_0.to_string()),
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse(&test_parse_option_configs);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_0() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_1() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "UNKNOWN".to_string(),
    },
    known: None,
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_2() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN=TEST_VALUE".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "UNKNOWN".to_string(),
    },
    known: None,
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_3() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN=".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "UNKNOWN".to_string(),
    },
    known: None,
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_4() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_5() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'U',
    },
    known: None,
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_6() {
  let test_args: Vec<String> = vec![
    "-T=TEST_VALUE".to_string(),
    "-U".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_7() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U=TEST_VALUE".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'U',
    },
    known: None,
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_8() {
  let test_args: Vec<String> = vec![
    "-T=".to_string(),
    "-U".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_9() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U=".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'U',
    },
    known: None,
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_10() {
  let test_args: Vec<String> = vec!["-TU".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_11() {
  let test_args: Vec<String> = vec!["-TU".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 1,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'U',
    },
    known: None,
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_12() {
  let test_args: Vec<String> = vec!["-UT".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 1,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_13() {
  let test_args: Vec<String> = vec!["-UT=TEST_VALUE".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 1,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_14() {
  let test_args: Vec<String> = vec!["-UT=".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 1,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_15() {
  let test_args: Vec<String> = vec!["-TU=TEST_VALUE".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 1,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'U',
    },
    known: None,
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_16() {
  let test_args: Vec<String> = vec!["-TU=".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 1,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'U',
    },
    known: None,
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_0() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN".to_string(),
  ];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Long {
        arg_index: 1,
        name_long: "UNKNOWN".to_string(),
      },
      known: None,
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unknown(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_1() {
  let test_args: Vec<String> = vec!["-TU".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 1,
        name_short: 'U',
      },
      known: None,
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unknown(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_2() {
  let test_args: Vec<String> = vec!["--".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Long {
        arg_index: 0,
        name_long: "".to_string(),
      },
      known: None,
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unknown(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_3() {
  let test_args: Vec<String> = vec!["-".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  // TODO: Should this be something else?
  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unknown(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_4() {
  let test_args: Vec<String> = vec!["--".to_string()];

  let test_recognized_options: &Vec<&ParseOptionConfig> =
    &vec![&TEST_PARSE_OPTION_CONFIG_1];

  let test_parse_input: ParseInput = ParseInput {
    args: &test_args,
    skip_arg: 0,
    skip_char: 0,
  };

  // TODO: Should this be something else?
  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unknown(test_recognized_options);

  assert_eq!(actual, expected);
}
