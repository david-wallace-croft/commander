//==============================================================================
//! Unit tests for module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-02
//! - Updated: 2024-07-30
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_error::ParseError;

use super::*;

const TEST_PARSE_OPTION_CONFIG_LONG: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_0",
  name: ParseOptionName::Long("TEST"),
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_OPTION: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_1",
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_REQUIRED: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_2",
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Required,
};

const TEST_PARSE_OPTION_CONFIG_SHORT: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_3",
  name: ParseOptionName::Short('T'),
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_VERBOTEN: ParseOptionConfig = ParseOptionConfig {
  id: "TEST_ID_4",
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Verboten,
};

//------------------------------------------------------------------------------
// make_hyphenated_option_name() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_make_hyphenated_option_name_0() {
  let expected: Option<String> = Some("--TEST".to_string());

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_LONG
    .make_hyphenated_option_name(HyphenationType::Long);

  assert_eq!(actual, expected);
}

#[test]
fn test_make_hyphenated_option_name_1() {
  let expected: Option<String> = None;

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_LONG
    .make_hyphenated_option_name(HyphenationType::Short);

  assert_eq!(actual, expected);
}

#[test]
fn test_make_hyphenated_option_name_2() {
  let expected: Option<String> = None;

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_SHORT
    .make_hyphenated_option_name(HyphenationType::Long);

  assert_eq!(actual, expected);
}

#[test]
fn test_make_hyphenated_option_name_3() {
  let expected: Option<String> = Some("-T".to_string());

  let actual: Option<String> = TEST_PARSE_OPTION_CONFIG_SHORT
    .make_hyphenated_option_name(HyphenationType::Short);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_0() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-T", "-X", "-T=A", "-Y", "-T=B", "-Z",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 1,
        char_index: 0,
        name_short: 'T',
      },
      known: Some("TEST_ID_1".to_string()),
      value: None,
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 3,
        char_index: 0,
        name_short: 'T',
      },
      known: Some("TEST_ID_1".to_string()),
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 5,
        char_index: 0,
        name_short: 'T',
      },
      known: Some("TEST_ID_1".to_string()),
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_1() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-X", "-Y", "-Z",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_2() {
  let test_args: Vec<String> = make_args(&[
    "-STAT=A", "-YT=B", "-Z",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 1,
        name_short: 'T',
      },
      known: Some("TEST_ID_1".to_string()),
      value: None,
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 3,
        name_short: 'T',
      },
      known: Some("TEST_ID_1".to_string()),
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 1,
        char_index: 1,
        name_short: 'T',
      },
      known: Some("TEST_ID_1".to_string()),
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse(&test_parse_input);

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

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 5,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: Some("B".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_1() {
  let test_args: Vec<String> = make_args(&[
    "-W", "-X", "-Y", "-Z",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = None;

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(&test_parse_input);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_next() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_next_option_0() {
  let test_args: Vec<String> = make_args(&[
    "-X", "-T", "value",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_1() {
  let test_args: Vec<String> = make_args(&["-T=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_2() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_3() {
  let test_args: Vec<String> = make_args(&["-T="]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_4() {
  let test_args: Vec<String> = make_args(&[
    "--EXCLUDE0",
    "--TEST",
    "value",
    "--EXCLUDE1",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_5() {
  let test_args: Vec<String> = make_args(&["--TEST=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_1".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_6() {
  let test_args: Vec<String> = make_args(&["--TEST"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_7() {
  let test_args: Vec<String> = make_args(&["--TEST="]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_8() {
  let test_args: Vec<String> = make_args(&["-XT=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_9() {
  let test_args: Vec<String> = make_args(&["-XTX=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some("TEST_ID_1".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_0() {
  let test_args: Vec<String> = make_args(&[
    "-T", "value",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_2".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_1() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_2".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_2() {
  let test_args: Vec<String> = make_args(&["-XT=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some("TEST_ID_2".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_3() {
  let test_args: Vec<String> = make_args(&["-XTX=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some("TEST_ID_2".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_multiple_0() {
  let test_args: Vec<String> = make_args(&[
    "-T=0", "-T=1",
  ]);

  let test_parse_input = ParseInput {
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
    known: Some("TEST_ID_2".to_string()),
    value: Some("0".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_multiple_1() {
  let test_args: Vec<String> = make_args(&[
    "-T=0", "-T=1",
  ]);

  let test_parse_input = ParseInput {
    args: &test_args,
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
    known: Some("TEST_ID_2".to_string()),
    value: Some("1".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_multiple_2() {
  let test_args: Vec<String> = make_args(&[
    "-T=0", "-T=1",
  ]);

  let test_parse_input = ParseInput {
    args: &test_args,
    skip_arg: usize::MAX,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = None;

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_0() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_1() {
  let test_args: Vec<String> = make_args(&[
    "-T", "value",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_2() {
  let test_args: Vec<String> = make_args(&["-T=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_4".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_3() {
  let test_args: Vec<String> = make_args(&["--TEST"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_4() {
  let test_args: Vec<String> = make_args(&[
    "--TEST", "value",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_5() {
  let test_args: Vec<String> = make_args(&["--TEST=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_4".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_6() {
  let test_args: Vec<String> = make_args(&["-T="]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_7() {
  let test_args: Vec<String> = make_args(&["--TEST="]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_8() {
  let test_args: Vec<String> = make_args(&["-XT=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some("TEST_ID_4".to_string()),
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_9() {
  let test_args: Vec<String> = make_args(&["-XTX=value"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    known: Some("TEST_ID_4".to_string()),
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(&test_parse_input);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// to_bool_result() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_to_bool_result_option_0() {
  let test_args: Vec<String> = make_args(&["-T=false"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_1() {
  let test_args: Vec<String> = make_args(&[
    "-T", "false",
  ]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_2() {
  let test_args: Vec<String> = make_args(&["-T=invalid"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_3() {
  let test_args: Vec<String> = make_args(&["--TEST=invalid"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_0() {
  let test_args: Vec<String> = make_args(&["-T=false"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_1() {
  let test_args: Vec<String> = make_args(&["-T=true"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_2() {
  let test_args: Vec<String> = make_args(&["-T=invalid"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_0() {
  let test_args: Vec<String> = make_args(&["-T"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_1() {
  let test_args: Vec<String> = make_args(&["--TEST"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_2() {
  let test_args: Vec<String> = make_args(&["-TEST"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_3() {
  let test_args: Vec<String> = make_args(&["-T=true"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> =
    Err(ParseError::VerbotenValuePresent);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_4() {
  let test_args: Vec<String> = make_args(&["--TEST=true"]);

  let test_parse_input = ParseInput::from_slice(&test_args);

  let expected: Result<bool, ParseError> =
    Err(ParseError::VerbotenValuePresent);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(&test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// supporting functions
//------------------------------------------------------------------------------

fn make_args(slice: &[&str]) -> Vec<String> {
  slice.iter().map(|s| s.to_string()).collect()
}
