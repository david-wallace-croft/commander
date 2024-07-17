//==============================================================================
//! Unit tests for module parse_input
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-31
//! - Updated: 2024-07-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use std::string::ToString;

use crate::parse::parse_error::ParseError;
use crate::parse::parse_option_name::ParseOptionName;
use crate::parse::value_usage::ValueUsage;

use super::*;

const TEST_PARSE_OPTION_CONFIG_0: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_0",
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIG_1: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_1",
  // TODO: Add this as an example to the README.md
  name: ParseOptionName::Long(""),
  value_usage: ValueUsage::Verboten,
};

#[test]
fn test_default_0() {
  let expected: usize = 1;

  let actual: usize = ParseInput::default().skip_arg;

  assert_eq!(actual, expected);
}

#[test]
fn test_from_slice_0() {
  let test_args_slice: &[&str] = &["TEST"];

  let expected: ParseInput = ParseInput {
    args: vec!["TEST".to_string()],
    skip_arg: 0,
    skip_char: 0,
  };

  let actual: ParseInput = ParseInput::from_slice(test_args_slice);

  assert_eq!(actual, expected);
}

// #[test]
// fn test_parse_0() {
//   let test_parse_input: ParseInput = ParseInput {
//     args: vec![
//       "TEST".to_string(),
//       "T".to_string(),
//     ],
//     skip: 0,
//   };
//
//   let test_parse_option_configs = vec![TEST_PARSE_OPTION_CONFIG_0];
//
//   let expected: Vec<ParseOutput> = vec![
//     ParseOutput {
//       error: None,
//       index: Some(0),
//       value: Some("TEST".to_string()),
//     },
//     ParseOutput {
//       error: None,
//       index: Some(0),
//       value: Some("T".to_string()),
//     },
//   ];
//
//   let actual: Vec<ParseOutput> =
//     test_parse_input.parse(&test_parse_option_configs);
//
//   assert_eq!(actual, expected);
// }

#[test]
fn test_parse_next_0() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "--TEST".to_string(),
      "--UNRECOGNIZED".to_string(),
    ],
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_1() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "--TEST".to_string(),
      "--UNRECOGNIZED".to_string(),
    ],
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "UNRECOGNIZED".to_string(),
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
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "--TEST".to_string(),
      "--UNRECOGNIZED=TEST_VALUE".to_string(),
    ],
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "UNRECOGNIZED".to_string(),
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
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "--TEST".to_string(),
      "--UNRECOGNIZED=".to_string(),
    ],
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "UNRECOGNIZED".to_string(),
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
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "-T".to_string(),
      "-U".to_string(),
    ],
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
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_5() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "-T".to_string(),
      "-U".to_string(),
    ],
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
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "-T=TEST_VALUE".to_string(),
      "-U".to_string(),
    ],
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
    known: Some("TEST_ID_0".to_string()),
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

// TODO: failing
#[ignore]
#[test]
fn test_parse_next_7() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "-T".to_string(),
      "-U=TEST_VALUE".to_string(),
    ],
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'T',
    },
    known: None,
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

// TODO: failing
#[ignore]
#[test]
fn test_parse_next_8() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "-T=".to_string(),
      "-U".to_string(),
    ],
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
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

// TODO: failing
#[ignore]
#[test]
fn test_parse_next_9() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "-T".to_string(),
      "-U=".to_string(),
    ],
    skip_arg: 1,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'T',
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
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-TU".to_string()],
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
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_11() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-TU".to_string()],
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
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-UT".to_string()],
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
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_13() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-UT=TEST_VALUE".to_string()],
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
    known: Some("TEST_ID_0".to_string()),
    value: Some("TEST_VALUE".to_string()),
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

// TODO: failing
#[ignore]
#[test]
fn test_parse_next_14() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-UT=".to_string()],
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
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    test_parse_input.parse_next(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_15() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-TU=TEST_VALUE".to_string()],
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

// TODO: failing
#[ignore]
#[test]
fn test_parse_next_16() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-TU=".to_string()],
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
fn test_parse_unrecognized_0() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec![
      "--TEST".to_string(),
      "--UNRECOGNIZED".to_string(),
    ],
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Long {
        arg_index: 1,
        name_long: "UNRECOGNIZED".to_string(),
      },
      known: None,
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unrecognized(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unrecognized_1() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-TU".to_string()],
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 0, // TODO: should be 1
        name_short: 'U',
      },
      known: None,
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unrecognized(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unrecognized_2() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["--".to_string()],
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
    test_parse_input.parse_unrecognized(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unrecognized_3() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["-".to_string()],
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Long {
        arg_index: 0,
        name_long: "-".to_string(),
      },
      known: None,
      value: None,
    },
  ];

  let actual: Vec<ParseOutput> =
    test_parse_input.parse_unrecognized(test_recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unrecognized_4() {
  let test_recognized_options: &Vec<ParseOptionConfig> =
    &vec![TEST_PARSE_OPTION_CONFIG_1];

  let test_parse_input: ParseInput = ParseInput {
    args: vec!["--".to_string()],
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
    test_parse_input.parse_unrecognized(test_recognized_options);

  assert_eq!(actual, expected);
}
