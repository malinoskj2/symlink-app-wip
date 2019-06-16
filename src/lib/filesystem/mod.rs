use std::fs;
use std::path::{Path, PathBuf};

use crate::error;
use crate::types::LinkData;
use crate::types::Linkfile;
use crate::FailErr;
use walkdir;

pub fn parse_linkfile<U: AsRef<Path>>(cfg_map: U) -> Result<Linkfile<LinkData>, FailErr> {
    debug!("parsing:{:?} ", cfg_map.as_ref());
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;
    Ok(cfg_map2)
}

pub fn parse(
    config_files: &[impl AsRef<Path>],
    tags: &[impl AsRef<str>],
) -> Result<Vec<Linkfile<LinkData>>, FailErr> {
    config_files
        .into_iter()
        .map(|path| parse_linkfile(path))
        .inspect(|res| match res.as_ref() {
            Ok(res) => error::handle_ok(res),
            Err(err) => error::handle_err(err),
        })
        .collect::<Result<Vec<Linkfile<LinkData>>, FailErr>>()
}

/// walk dirs recursively looking for name
pub fn find_config(dir: impl AsRef<Path>, config_name: &str) -> Option<PathBuf> {
    walkdir::WalkDir::new(dir.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| {
            if let Some(file_name) = e.file_name().to_str() {
                file_name == config_name
            } else {
                false
            }
        })
        .map(|e| e.into_path())
}
