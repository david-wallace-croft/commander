//==============================================================================
//! Unit tests for module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-02
//! - Updated: 2025-11-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::*;
use crate::parse::parse_error::ParseError;
use crate::parse::parse_found::ParseFound;

const TEST_ID_0: &str = "TEST_ID_0";
const TEST_ID_1: &str = "TEST_ID_1";
const TEST_ID_2: &str = "TEST_ID_2";

const PARSE_OPTION_CONFIG_OPTION: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_0,
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_REQUIRED: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_1,
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Required,
};

const PARSE_OPTION_CONFIG_VERBOTEN: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_2,
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Verboten,
};

//------------------------------------------------------------------------------
// parse() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_0() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-T", "-X", "-T=A", "-Y", "-T=B", "-Z",
  ]);

  let expected: Vec<ParseOutput> = vec![
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
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 3,
        char_index: 0,
        name_short: 'T',
      },
      known: Some(TEST_ID_0.to_string()),
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 5,
        char_index: 0,
        name_short: 'T',
      },
      known: Some(TEST_ID_0.to_string()),
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> = PARSE_OPTION_CONFIG_OPTION.parse(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_1() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-X", "-Y", "-Z",
  ]);

  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> = PARSE_OPTION_CONFIG_OPTION.parse(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_2() {
  let test_args: Vec<String> = make_args(&[
    "-STAT=A", "-YT=B", "-Z",
  ]);

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 1,
        name_short: 'T',
      },
      known: Some(TEST_ID_0.to_string()),
      value: None,
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 3,
        name_short: 'T',
      },
      known: Some(TEST_ID_0.to_string()),
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 1,
        char_index: 1,
        name_short: 'T',
      },
      known: Some(TEST_ID_0.to_string()),
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> = PARSE_OPTION_CONFIG_OPTION.parse(&test_args);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_last() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_last_0() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-T", "-X", "-T=A", "-Y", "-T=B", "-Z",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 5,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: Some("B".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_1() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-X", "-Y", "-Z",
  ]);

  let expected: Option<ParseOutput> = None;

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_last() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_last_option_0() {
  let test_args: Vec<String> = make_args(&[
    "-X", "-T", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_1() {
  let test_args: Vec<String> = make_args(&["-T=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_2() {
  let test_args: Vec<String> = make_args(&["-T"]);

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
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_3() {
  let test_args: Vec<String> = make_args(&["-T="]);

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
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_4() {
  let test_args: Vec<String> = make_args(&[
    "--EXCLUDE0",
    "--TEST",
    "value",
    "--EXCLUDE1",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_5() {
  let test_args: Vec<String> = make_args(&["--TEST=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_0.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_6() {
  let test_args: Vec<String> = make_args(&["--TEST"]);

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
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_7() {
  let test_args: Vec<String> = make_args(&["--TEST="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_0.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_8() {
  let test_args: Vec<String> = make_args(&["-XT=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_0.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_option_9() {
  let test_args: Vec<String> = make_args(&["-XTX=value"]);

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
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_required_0() {
  let test_args: Vec<String> = make_args(&[
    "-T", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_1.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_required_1() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_1.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_required_2() {
  let test_args: Vec<String> = make_args(&["-XT=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_1.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_required_3() {
  let test_args: Vec<String> = make_args(&["-XTX=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_1.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);

  assert_eq!(actual, expected);
}

// #[test]
// fn test_parse_last_required_multiple_0() {
//   let test_args: Vec<String> = make_args(&[
//     "-T=0", "-T=1",
//   ]);
//
//   let expected: Option<ParseOutput> = Some(ParseOutput {
//     error: None,
//     found: ParseFound::Short {
//       arg_index: 0,
//       char_index: 0,
//       name_short: 'T',
//     },
//     known: Some(TEST_ID_1.to_string()),
//     value: Some("0".to_string()),
//   });
//
//   let actual: Option<ParseOutput> =
//     PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);
//
//   assert_eq!(actual, expected);
// }

#[test]
fn test_parse_last_required_multiple_1() {
  let test_args: Vec<String> = make_args(&[
    "-T=0", "-T=1",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_1.to_string()),
    value: Some("1".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);

  assert_eq!(actual, expected);
}

// #[test]
// fn test_parse_last_required_multiple_2() {
//   let test_args: Vec<String> = make_args(&[
//     "-T=0", "-T=1",
//   ]);
//
//   let expected: Option<ParseOutput> = None;
//
//   let actual: Option<ParseOutput> =
//     PARSE_OPTION_CONFIG_REQUIRED.parse_last(&test_args);
//
//   assert_eq!(actual, expected);
// }

#[test]
fn test_parse_last_verboten_0() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_1() {
  let test_args: Vec<String> = make_args(&[
    "-T", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_2() {
  let test_args: Vec<String> = make_args(&["-T=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_2.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_3() {
  let test_args: Vec<String> = make_args(&["--TEST"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_4() {
  let test_args: Vec<String> = make_args(&[
    "--TEST", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_5() {
  let test_args: Vec<String> = make_args(&["--TEST=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_2.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_6() {
  let test_args: Vec<String> = make_args(&["-T="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_7() {
  let test_args: Vec<String> = make_args(&["--TEST="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_8() {
  let test_args: Vec<String> = make_args(&["-XT=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_2.to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_verboten_9() {
  let test_args: Vec<String> = make_args(&["-XTX=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some(TEST_ID_2.to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_last(&test_args);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// to_bool_result() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_to_bool_result_option_0() {
  let test_args: Vec<String> = make_args(&["-T=false"]);

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_1() {
  let test_args: Vec<String> = make_args(&[
    "-T", "false",
  ]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_2() {
  let test_args: Vec<String> = make_args(&["-T=invalid"]);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_3() {
  let test_args: Vec<String> = make_args(&["--TEST=invalid"]);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_0() {
  let test_args: Vec<String> = make_args(&["-T=false"]);

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_1() {
  let test_args: Vec<String> = make_args(&["-T=true"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_2() {
  let test_args: Vec<String> = make_args(&["-T=invalid"]);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_0() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_1() {
  let test_args: Vec<String> = make_args(&["--TEST"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_2() {
  let test_args: Vec<String> = make_args(&["-TEST"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_3() {
  let test_args: Vec<String> = make_args(&["-T=true"]);

  let expected: Result<bool, ParseError> =
    Err(ParseError::VerbotenValuePresent);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_4() {
  let test_args: Vec<String> = make_args(&["--TEST=true"]);

  let expected: Result<bool, ParseError> =
    Err(ParseError::VerbotenValuePresent);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_last(&test_args)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// supporting functions
//------------------------------------------------------------------------------

fn make_args(slice: &[&str]) -> Vec<String> {
  slice.iter().map(|s: &&str| s.to_string()).collect()
}
