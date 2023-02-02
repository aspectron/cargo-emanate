use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error: {0}")]
    String(String),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("OsString error: {0}")]
    OsString(String),

    #[error("StripPrefixError: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("invalid version {0}")]
    InvalidVersion(String),

    #[error(transparent)]
    CratesIoApi(#[from] crates_io_api::Error),
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => (
        Error::String(format!("{}",&format_args!($($t)*)))
    )
}

pub use error;
