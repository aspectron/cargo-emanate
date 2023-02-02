use crate::prelude::*;

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}
