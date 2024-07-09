//==============================================================================
//! Unit tests for module parse_option_config
//!
//! # Metadata
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-06-02
//! - Updated: 2024-07-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use crate::parse::parse_error::ParseError;
use crate::parse::parse_input::ParseUnrecognizedOutput;

use super::*;

const TEST_PARSE_OPTION_CONFIG_LONG: ParseOptionConfig = ParseOptionConfig {
  name: ParseOptionName::Long("TEST"),
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_OPTION: ParseOptionConfig = ParseOptionConfig {
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_REQUIRED: ParseOptionConfig = ParseOptionConfig {
  name: ParseOptionName::Both {
    name_long: "TEST",
    name_short: 'T',
  },
  value_usage: ValueUsage::Required,
};

const TEST_PARSE_OPTION_CONFIG_SHORT: ParseOptionConfig = ParseOptionConfig {
  name: ParseOptionName::Short('T'),
  value_usage: ValueUsage::Optional,
};

const PARSE_OPTION_CONFIG_VERBOTEN: ParseOptionConfig = ParseOptionConfig {
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
  let test_parse_input = &ParseInput::from_slice(&[
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
      value: None,
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 3,
        char_index: 0,
        name_short: 'T',
      },
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 5,
        char_index: 0,
        name_short: 'T',
      },
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-W", "-X", "-Y", "-Z",
  ]);

  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_2() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-WTXT=A", "-YT=B", "-Z",
  ]);

  let expected: Vec<ParseOutput> = vec![
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 1,
        name_short: 'T',
      },
      value: None,
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 0,
        char_index: 3,
        name_short: 'T',
      },
      value: Some("A".to_string()),
    },
    ParseOutput {
      error: None,
      found: ParseFound::Short {
        arg_index: 1,
        char_index: 1,
        name_short: 'T',
      },
      value: Some("B".to_string()),
    },
  ];

  let actual: Vec<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse(test_parse_input);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_hyphenated_option_name() unit tests
//------------------------------------------------------------------------------

// TODO: Convert these old tests to parse_long() unit tests?

// #[test]
// fn test_parse_hyphenated_option_name_0() {
//   let expected: Option<ParseOutput> = None;
//
//   let actual: Option<ParseOutput> =
//     ParseOptionConfig::parse_hyphenated_option_name(
//       "--UNRECOGNIZED",
//       0,
//       "--TEST",
//       ValueUsage::Optional,
//     );
//
//   assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_parse_hyphenated_option_name_1() {
//   let expected: Option<ParseOutput> = Some(ParseOutput {
//     error: None,
//     index: 0,
//     value: None,
//   });
//
//   let actual: Option<ParseOutput> =
//     ParseOptionConfig::parse_hyphenated_option_name(
//       "--TEST",
//       0,
//       "--TEST",
//       ValueUsage::Optional,
//     );
//
//   assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_parse_hyphenated_option_name_2() {
//   let expected: Option<ParseOutput> = Some(ParseOutput {
//     error: Some(ParseError::ValueMissingAfterEquals),
//     index: 0,
//     value: None,
//   });
//
//   let actual: Option<ParseOutput> =
//     ParseOptionConfig::parse_hyphenated_option_name(
//       "--TEST=",
//       0,
//       "--TEST",
//       ValueUsage::Optional,
//     );
//
//   assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_parse_hyphenated_option_name_3() {
//   let expected: Option<ParseOutput> = Some(ParseOutput {
//     error: None,
//     index: 0,
//     value: Some("VALUE".to_string()),
//   });
//
//   let actual: Option<ParseOutput> =
//     ParseOptionConfig::parse_hyphenated_option_name(
//       "--TEST=VALUE",
//       0,
//       "--TEST",
//       ValueUsage::Optional,
//     );
//
//   assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_parse_hyphenated_option_name_4() {
//   let expected: Option<ParseOutput> = Some(ParseOutput {
//     error: Some(ParseError::VerbotenValuePresent),
//     index: 0,
//     value: Some("VALUE".to_string()),
//   });
//
//   let actual: Option<ParseOutput> =
//     ParseOptionConfig::parse_hyphenated_option_name(
//       "--TEST=VALUE",
//       0,
//       "--TEST",
//       ValueUsage::Verboten,
//     );
//
//   assert_eq!(actual, expected);
// }
//
// #[test]
// fn test_parse_hyphenated_option_name_5() {
//   let expected: Option<ParseOutput> = Some(ParseOutput {
//     error: Some(ParseError::RequiredValueMissing),
//     index: 0,
//     value: None,
//   });
//
//   let actual: Option<ParseOutput> =
//     ParseOptionConfig::parse_hyphenated_option_name(
//       "--TEST",
//       0,
//       "--TEST",
//       ValueUsage::Required,
//     );
//
//   assert_eq!(actual, expected);
// }

//------------------------------------------------------------------------------
// parse_last() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_last_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-W", "-T", "-X", "-T=A", "-Y", "-T=B", "-Z",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 5,
      char_index: 0,
      name_short: 'T',
    },
    value: Some("B".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_last_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-W", "-X", "-Y", "-Z",
  ]);

  let expected: Option<ParseOutput> = None;

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_last(test_parse_input);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_next() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_next_option_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-X", "-T", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 1,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_3() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_4() {
  let test_parse_input = &ParseInput::from_slice(&[
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
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_6() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_8() {
  let test_parse_input = &ParseInput::from_slice(&["-XT=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_option_9() {
  let test_parse_input = &ParseInput::from_slice(&["-XTX=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_OPTION.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_0() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_2() {
  let test_parse_input = &ParseInput::from_slice(&["-XT=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_3() {
  let test_parse_input = &ParseInput::from_slice(&["-XTX=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::RequiredValueMissing),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_multiple_0() {
  let test_parse_input = &ParseInput {
    args: vec![
      "-T=0".to_string(),
      "-T=1".to_string(),
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
    value: Some("0".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_multiple_1() {
  let test_parse_input = &ParseInput {
    args: vec![
      "-T=0".to_string(),
      "-T=1".to_string(),
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
    value: Some("1".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_required_multiple_2() {
  let test_parse_input = &ParseInput {
    args: vec![
      "-T=0".to_string(),
      "-T=1".to_string(),
    ],
    skip_arg: usize::MAX,
    skip_char: 0,
  };

  let expected: Option<ParseOutput> = None;

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_REQUIRED.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_4() {
  let test_parse_input = &ParseInput::from_slice(&[
    "--TEST", "value",
  ]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_5() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_6() {
  let test_parse_input = &ParseInput::from_slice(&["-T="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_7() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST="]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::ValueMissingAfterEquals),
    found: ParseFound::Long {
      arg_index: 0,
      name_long: "TEST".to_string(),
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_8() {
  let test_parse_input = &ParseInput::from_slice(&["-XT=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: Some(ParseError::VerbotenValuePresent),
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: Some("value".to_string()),
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_verboten_9() {
  let test_parse_input = &ParseInput::from_slice(&["-XTX=value"]);

  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_next(test_parse_input);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_short() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_parse_short_0() {
  let expected: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 1,
      name_short: 'T',
    },
    value: None,
  });

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_short("-XTX=value", 0, 1);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_short_1() {
  let expected: Option<ParseOutput> = None;

  let actual: Option<ParseOutput> =
    PARSE_OPTION_CONFIG_VERBOTEN.parse_short("-XTX=value", 0, 2);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// parse_unrecognized() unit tests
//------------------------------------------------------------------------------

// TODO: Should these tests be moved to another test module?

#[test]
fn test_parse_unrecognized_long() {
  const ARG_OPTION_TEST: ParseOptionConfig = ParseOptionConfig {
    name: ParseOptionName::Both {
      name_long: "TEST",
      name_short: 'T',
    },
    value_usage: ValueUsage::Optional,
  };

  let recognized_options: Vec<ParseOptionConfig> = vec![ARG_OPTION_TEST];

  let test_parse_input = &ParseInput::from_slice(&["--unrecognized"]);

  let expected: Vec<ParseUnrecognizedOutput> = vec![
    ParseUnrecognizedOutput {
      index: 0,
      name: "unrecognized".to_string(),
    },
  ];

  let actual: Vec<ParseUnrecognizedOutput> =
    test_parse_input.parse_unrecognized(&recognized_options);

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unrecognized_short() {
  const ARG_OPTION_TEST: ParseOptionConfig = ParseOptionConfig {
    name: ParseOptionName::Both {
      name_long: "TEST",
      name_short: 'T',
    },
    value_usage: ValueUsage::Optional,
  };

  let recognized_options: Vec<ParseOptionConfig> = vec![ARG_OPTION_TEST];

  let test_parse_input = &ParseInput::from_slice(&["-u"]);

  let expected: Vec<ParseUnrecognizedOutput> = vec![
    ParseUnrecognizedOutput {
      index: 0,
      name: "u".to_string(),
    },
  ];

  let actual: Vec<ParseUnrecognizedOutput> =
    test_parse_input.parse_unrecognized(&recognized_options);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// to_bool_result() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_to_bool_result_option_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_1() {
  let test_parse_input = &ParseInput::from_slice(&[
    "-T", "false",
  ]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_option_3() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=invalid"]);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_OPTION
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T=false"]);

  let expected: Result<bool, ParseError> = Ok(false);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_1() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_required_2() {
  let test_parse_input = &ParseInput::from_slice(&["-T=invalid"]);

  let expected: Result<bool, ParseError> = Err(ParseError::InvalidValue);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_REQUIRED
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_0() {
  let test_parse_input = &ParseInput::from_slice(&["-T"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_1() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_2() {
  let test_parse_input = &ParseInput::from_slice(&["-TEST"]);

  let expected: Result<bool, ParseError> = Ok(true);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_3() {
  let test_parse_input = &ParseInput::from_slice(&["-T=true"]);

  let expected: Result<bool, ParseError> =
    Err(ParseError::VerbotenValuePresent);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}

#[test]
fn test_to_bool_result_verboten_4() {
  let test_parse_input = &ParseInput::from_slice(&["--TEST=true"]);

  let expected: Result<bool, ParseError> =
    Err(ParseError::VerbotenValuePresent);

  let actual: Result<bool, ParseError> = PARSE_OPTION_CONFIG_VERBOTEN
    .parse_next(test_parse_input)
    .unwrap()
    .to_bool_result();

  assert_eq!(actual, expected);
}
