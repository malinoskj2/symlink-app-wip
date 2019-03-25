#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::hash::Hash;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub type DIResult<T> = Result<T, Box<Error>>;

const CFG_MAP_NAME: &str = "config-map.yaml";

pub fn install(repo_path: impl AsRef<Path>) -> HashMap<String, ConfigLink> {
    find_config_map(repo_path)
        .iter()
        .flat_map(|path: &PathBuf| parse_config_links(path))
        .collect()
}

pub fn find_config_map(repo_path: impl AsRef<Path>) -> Vec<PathBuf> {
    println!("finding config map");
    WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name() == OsString::from(CFG_MAP_NAME))
        .map(DirEntry::into_path)
        .collect()
}

pub fn parse_config_links(cfg_map: impl AsRef<Path>) -> HashMap<String, ConfigLink> {
    fs::File::open(cfg_map)
        .map(|file| {
            let res_map: HashMap<String, ConfigLink> =
                serde_yaml::from_reader(file).expect("failed to read cfg");
            res_map
        })
        .expect("failed to read cfg")
}

#[derive(Debug, Serialize, Deserialize)]
enum CLMethod {
    link,
    copy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLink {
    source: String,
    destination: String,
    method: String,
}
