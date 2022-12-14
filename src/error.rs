use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Error: {0}")]
    String(String),

    #[error("Warning: {0}")]
    Warning(String),
    
    // #[error("Unknown architecture: '{0}'")]
    // InvalidArchitecture(String),
    
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("OsString error: {0}")]
    OsString(String),
    
    // #[error("FileSystem error: {0}")]
    // FsExtra(#[from] fs_extra::error::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    
    // #[error("Regex error: {0}")]
    // Regex(#[from] regex::Error),
    
    #[error("StripPrefixError: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Cargo.toml manifest: {0}")]
    CargoManifest(#[from] cargo_toml::Error),
    
    // #[error("Cargo.toml manifest: {0}")]
    // InheritedUnknownValue(#[from] cargo_toml::ErrInheritedUnknownValue),

    #[error("invalid version {0}")]
    InvalidVersion(String),
}


#[macro_export]
macro_rules! error {
    ($($t:tt)*) => (
        Error::String(format!("{}",&format_args!($($t)*)))
    )
}

pub use error;