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
        T: Into<std::ffi::OsString>,
    {
        todo!()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingPattern,
    MissingPath,
}
