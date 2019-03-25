use std::ffi::OsString;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_file_in_dir(dir_path: impl AsRef<Path>, comparator: &str) -> Vec<PathBuf> {
    println!("finding config map");
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name() == OsString::from(comparator))
        .map(DirEntry::into_path)
        .collect()
}
