use std::ffi::{OsStr, OsString};
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

use crate::FailErr;

use super::error::InstallerErr;

pub fn find_file_in_dir<T: AsRef<str>, U: AsRef<Path>>(
    dir_path: U,
    comparator: &[T],
    sub_directories: &[U],
) -> Result<Vec<DirEntry>, walkdir::Error> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_entry(|entry| compare_paths(entry.path(), sub_directories))
        .filter(|entry| entry.is_ok())
        .filter(|entry| check_entry(&entry, comparator).unwrap())
        .collect::<Result<Vec<DirEntry>, walkdir::Error>>()
}

fn compare_paths<U: AsRef<Path>>(walked: &Path, sub_directories: &[U]) -> bool {
    if sub_directories.len() == 0 {
        true
    } else {
        sub_directories
            .iter()
            .any(|allowed| allowed.as_ref().starts_with(walked) || walked.starts_with(allowed))
    }
}

// Check to see if an entry matches what we are looking fork
fn check_entry<T: AsRef<str>>(
    entry: &Result<DirEntry, walkdir::Error>,
    compare_strings: &[T],
) -> Result<bool, FailErr> {
    let entry_str = entry.as_ref().unwrap().to_owned();
    let entry_str = entry_str.file_name().to_str().unwrap();

    Ok(compare_strings
        .into_iter()
        .any(|comp_str| entry_str == comp_str.as_ref()))
}
