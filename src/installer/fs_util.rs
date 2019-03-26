use super::error::InstallerErr;
use crate::FailErr;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_file_in_dir(
    dir_path: impl AsRef<Path>,
    comparator: Vec<String>,
) -> Result<Vec<PathBuf>, FailErr> {
    let res_vec: Vec<PathBuf> = WalkDir::new(dir_path)
        .into_iter()
        .filter(|entry| entry.is_ok())
        .filter(|entry| check_entry(entry, &comparator).unwrap_or(false))
        .filter_map(|e| e.ok())
        .map(DirEntry::into_path)
        .collect();

    Ok(res_vec)
}

// Check to see if an entry matches what we are looking fork
fn check_entry(
    entry: &Result<DirEntry, walkdir::Error>,
    compare_strings: &Vec<String>,
) -> Result<bool, FailErr> {
    let entry_str = entry.as_ref().unwrap().to_owned();
    let entry_str = entry_str.file_name().to_str().unwrap();

    Ok(compare_strings
        .into_iter()
        .any(|comp_str| entry_str == comp_str))
}

pub fn ff_in_dir(dir_path: impl AsRef<Path>, comparator: &str) -> Result<Vec<PathBuf>, FailErr> {
    let res_vec: Vec<PathBuf> = WalkDir::new(dir_path)
        .into_iter()
        .filter(|entry| entry.is_ok())
        .filter(|entry| check_entry(entry, &vec![comparator.to_string()]).unwrap_or(false))
        .filter_map(|e| e.ok())
        .map(DirEntry::into_path)
        .collect();

    Ok(res_vec)
}
