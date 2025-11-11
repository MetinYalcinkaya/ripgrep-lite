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

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingPattern,
    MissingPath,
}
