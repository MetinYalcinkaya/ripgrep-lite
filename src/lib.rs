use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub paths: Vec<std::path::PathBuf>,
    pub ignore_case: bool,
    pub count_only: bool,
    pub line_numbers: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        todo!()
    }

    pub fn from_args<I, T>(args: I) -> Result<Self, ConfigError>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let mut args_iter = args.into_iter().skip(1);

        let pattern = args_iter.next().ok_or(ConfigError::MissingPattern)?.into();
        let first_path = args_iter.next().ok_or(ConfigError::MissingPath)?.into();

        let mut paths = vec![PathBuf::from(first_path)];
        paths.extend(args_iter.map(|t| PathBuf::from(t.into())));

        Ok(Config {
            pattern,
            paths,
            ignore_case: false,
            count_only: false,
            line_numbers: false,
        })
    }
}

pub fn search<'a>(pattern: &str, input: &'a str) -> Vec<(usize, &'a str)> {
    let mut matches: Vec<(usize, &str)> = vec![];
    for (line_no, line) in input.lines().enumerate() {
        if line.contains(pattern) {
            matches.push((line_no + 1, line));
        }
    }
    matches
}

#[test]
fn search_test_matching_lines() {
    let pattern = "testing";
    let input = "\
This is the testing
kind of test
where we do testing
for the tests
of testing";
    let output = [
        (1, "This is the testing"),
        (3, "where we do testing"),
        (5, "of testing"),
    ];

    assert_eq!(output, *search(pattern, input));
}

#[test]
fn search_test_no_matches() {
    let pattern = "nothing";
    let input = "\
This is the testing
kind of test
where we do testing
for the tests
of testing";
    let output: Vec<(usize, &str)> = vec![];

    assert_eq!(output, *search(pattern, input));
}

#[test]
fn search_test_no_pattern() {
    let pattern = "";
    let input = "\
This is the testing
kind of test
where we do testing
for the tests
of testing";
    let output = [
        (1, "This is the testing"),
        (2, "kind of test"),
        (3, "where we do testing"),
        (4, "for the tests"),
        (5, "of testing"),
    ];

    assert_eq!(output, *search(pattern, input));
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingPattern,
    MissingPath,
}
