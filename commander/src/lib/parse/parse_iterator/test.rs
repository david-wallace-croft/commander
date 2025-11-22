//==============================================================================
//! Unit tests for module parse_iterator
//!
//! # Metadata
//! - Copyright: &copy; 2024-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2024-05-31
//! - Updated: 2025-11-21
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use std::string::ToString;
use std::sync::LazyLock;

use crate::parse::parse_error::ParseError;
use crate::parse::parse_option_name::ParseOptionName;
use crate::parse::value_usage::ValueUsage;

use super::*;

static TEST_ARGS_0: LazyLock<Vec<String>> = LazyLock::new(|| {
  [
    "-T", "--TEST",
  ]
  .iter()
  .map(|s: &&str| s.to_string())
  .collect()
});

const TEST_ID_0: &str = "TEST_ID_0";
const TEST_ID_1: &str = "TEST_ID_1";
const TEST_ID_2: &str = "TEST_ID_2";
const TEST_ID_3: &str = "TEST_ID_3";

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

const TEST_PARSE_OPTION_CONFIG_LONG: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_2,
  name: ParseOptionName::Long("TEST"),
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIG_SHORT: ParseOptionConfig = ParseOptionConfig {
  id: TEST_ID_3,
  name: ParseOptionName::Short('T'),
  value_usage: ValueUsage::Optional,
};

const TEST_PARSE_OPTION_CONFIGS_0: &[&ParseOptionConfig] =
  &[&TEST_PARSE_OPTION_CONFIG_0];

#[test]
fn test_collect_0() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "-T".to_string(),
  ];

  let test_parse_option_configs: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator<'_> = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_parse_option_configs,
    skip_arg: 0,
    skip_char: 0,
  };

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

  let actual: Vec<ParseOutput> = test_parse_iterator.collect();

  assert_eq!(actual, expected);
}

#[test]
fn test_from_slice_0() {
  let test_args_slice: Vec<String> = vec!["TEST".to_string()];

  let args: &Vec<String> = &vec!["TEST".to_string()];

  let expected: ParseIterator = ParseIterator {
    args,
    parse_option_configs: &[],
    skip_arg: 0,
    skip_char: 0,
  };

  let actual: ParseIterator = ParseIterator::from_slice(&test_args_slice);

  assert_eq!(actual, expected);
}

//------------------------------------------------------------------------------
// make_hyphenated_option_name() unit tests
//------------------------------------------------------------------------------

#[test]
fn test_make_hyphenated_option_name_0() {
  let expected: Option<String> = Some("--TEST".to_string());

  let actual: Option<String> = ParseIterator::make_hyphenated_option_name(
    HyphenationType::Long,
    &TEST_PARSE_OPTION_CONFIG_LONG,
  );

  assert_eq!(actual, expected);
}

#[test]
fn test_make_hyphenated_option_name_1() {
  let expected: Option<String> = None;

  let actual: Option<String> = ParseIterator::make_hyphenated_option_name(
    HyphenationType::Short,
    &TEST_PARSE_OPTION_CONFIG_LONG,
  );

  assert_eq!(actual, expected);
}

#[test]
fn test_make_hyphenated_option_name_2() {
  let expected: Option<String> = None;

  let actual: Option<String> = ParseIterator::make_hyphenated_option_name(
    HyphenationType::Long,
    &TEST_PARSE_OPTION_CONFIG_SHORT,
  );

  assert_eq!(actual, expected);
}

#[test]
fn test_make_hyphenated_option_name_3() {
  let expected: Option<String> = Some("-T".to_string());

  let actual: Option<String> = ParseIterator::make_hyphenated_option_name(
    HyphenationType::Short,
    &TEST_PARSE_OPTION_CONFIG_SHORT,
  );

  assert_eq!(actual, expected);
}

#[test]
fn test_next_0() {
  let mut parse_iterator: ParseIterator<'_> = ParseIterator {
    args: &TEST_ARGS_0,
    parse_option_configs: TEST_PARSE_OPTION_CONFIGS_0,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected0: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let expected1: Option<ParseOutput> = Some(ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  });

  let expected3: Option<ParseOutput> = None;

  let actual0: Option<ParseOutput> = parse_iterator.next();

  assert_eq!(actual0, expected0);

  let actual1: Option<ParseOutput> = parse_iterator.next();

  assert_eq!(actual1, expected1);

  let actual2: Option<ParseOutput> = parse_iterator.next();

  assert_eq!(actual2, expected3);
}

#[test]
fn test_next_1() {
  let parse_iterator: ParseIterator<'_> = ParseIterator {
    args: &TEST_ARGS_0,
    parse_option_configs: TEST_PARSE_OPTION_CONFIGS_0,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected0: ParseOutput = ParseOutput {
    error: None,
    found: ParseFound::Short {
      arg_index: 0,
      char_index: 0,
      name_short: 'T',
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  };

  let expected1: ParseOutput = ParseOutput {
    error: None,
    found: ParseFound::Long {
      arg_index: 1,
      name_long: "TEST".to_string(),
    },
    known: Some("TEST_ID_0".to_string()),
    value: None,
  };

  let expected: Vec<ParseOutput> = vec![
    expected0, expected1,
  ];

  let mut actual: Vec<ParseOutput> = Vec::new();

  for parse_output in parse_iterator {
    actual.push(parse_output);
  }

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_0() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_1() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_2() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN=TEST_VALUE".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_3() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN=".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_4() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_5() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_6() {
  let test_args: Vec<String> = vec![
    "-T=TEST_VALUE".to_string(),
    "-U".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_7() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U=TEST_VALUE".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_8() {
  let test_args: Vec<String> = vec![
    "-T=".to_string(),
    "-U".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_9() {
  let test_args: Vec<String> = vec![
    "-T".to_string(),
    "-U=".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_10() {
  let test_args: Vec<String> = vec!["-TU".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_11() {
  let test_args: Vec<String> = vec!["-TU".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_12() {
  let test_args: Vec<String> = vec!["-UT".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_13() {
  let test_args: Vec<String> = vec!["-UT=TEST_VALUE".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_14() {
  let test_args: Vec<String> = vec!["-UT=".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_15() {
  let test_args: Vec<String> = vec!["-TU=TEST_VALUE".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_next_16() {
  let test_args: Vec<String> = vec!["-TU=".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Option<ParseOutput> = test_parse_iterator.parse_next();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_0() {
  let test_args: Vec<String> = vec![
    "--TEST".to_string(),
    "--UNKNOWN".to_string(),
  ];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let mut test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Vec<ParseOutput> = test_parse_iterator.parse_unknown();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_1() {
  let test_args: Vec<String> = vec!["-TU".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let mut test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Vec<ParseOutput> = test_parse_iterator.parse_unknown();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_2() {
  let test_args: Vec<String> = vec!["--".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let mut test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
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

  let actual: Vec<ParseOutput> = test_parse_iterator.parse_unknown();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_3() {
  let test_args: Vec<String> = vec!["-".to_string()];

  let test_known_options: Vec<&ParseOptionConfig<'_>> =
    vec![&TEST_PARSE_OPTION_CONFIG_0];

  let mut test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> = test_parse_iterator.parse_unknown();

  assert_eq!(actual, expected);
}

#[test]
fn test_parse_unknown_4() {
  let test_args: Vec<String> = vec!["--".to_string()];

  let test_known_options: Vec<&ParseOptionConfig> =
    vec![&TEST_PARSE_OPTION_CONFIG_1];

  let mut test_parse_iterator: ParseIterator = ParseIterator {
    args: &test_args,
    parse_option_configs: &test_known_options,
    skip_arg: 0,
    skip_char: 0,
  };

  let expected: Vec<ParseOutput> = vec![];

  let actual: Vec<ParseOutput> = test_parse_iterator.parse_unknown();

  assert_eq!(actual, expected);
}
