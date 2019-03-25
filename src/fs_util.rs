use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_file_in_dir(dir_path: impl AsRef<Path>, comparator: Vec<String>) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            comparator
                .iter()
                .any(|comp_str| entry.file_name().to_str().unwrap() == comp_str)
        })
        .map(DirEntry::into_path)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_cfg() {
        let expected = PathBuf::from("/usr/home/jesse/env/package/cargo.yaml");

        let found_paths = find_file_in_dir(
            PathBuf::from("/usr/home/jesse/env"),
            vec![String::from("cargo.yaml")],
        );

        found_paths.iter().for_each(|path| {
            assert_eq!(&expected, path);
        });
    }
}
