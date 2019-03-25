use std::error::Error;
use std::path::{Path, PathBuf};

mod linker;

pub type DIResult<T> = Result<T, Box<Error>>;

pub fn install(repo_path: impl AsRef<Path>) -> Vec<PathBuf> {
    let config_map_paths: Vec<PathBuf> = linker::find_config_map(repo_path.as_ref());
    return config_map_paths;
}
