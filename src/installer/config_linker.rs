use std::fs;
use std::path::{Path, PathBuf};

use crate::FailErr;

use super::error::InstallerErr;
use super::fs_util;
use indicatif::{ProgressBar, ProgressStyle};
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub fn install<T: AsRef<str>, U: AsRef<Path>>(
    repo_path: U,
    cfg_names: &[T],
    sub_directories: &[U],
    tags: &[T],
) -> Result<(), FailErr> {
    fs_util::find_file_in_dir(repo_path.into(), cfg_names.into(), sub_directories.into())
        .iter()
        .flat_map(|dir_result| {
            dir_result.as_ref().and_then(|dir| {
                let res_map: Result<ConfigMap<ConfigLink>, &InstallerErr> =
                    parse_config_map(dir.path()).map_err(|_| &InstallerErr::NoPath);

                res_map
            })
        })
        .filter(|cfg_map_res| apply_tag_filter(&cfg_map_res, tags))
        .fold(Ok(()), |_, map| map.create_links());

    Ok(())
}

fn apply_tag_filter<T: AsRef<str>, U: Linkable>(cfg_map: &ConfigMap<U>, tags: &[T]) -> bool {
    if tags.len() == 0 {
        true
    } else {
        tags.iter()
            .any(|tag_name| cfg_map.has_tag(tag_name.as_ref()))
    }
}

fn parse_config_map<T: DeserializeOwned + Linkable, U: AsRef<Path>>(
    cfg_map: U,
) -> Result<ConfigMap<T>, FailErr> {
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;

    Ok(cfg_map2)
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigMap<T: Linkable> {
    tags: Vec<String>,
    links: Vec<T>,
}

impl<T: Linkable> ConfigMap<T> {
    fn has_tag(&self, tag_name: &str) -> bool {
        self.tags.iter().any(|tag| tag == tag_name)
    }

    fn create_links(&self) -> Result<(), FailErr> {
        let mut bar = ProgressBar::new((self.links.len()) as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .progress_chars("##-"),
        );

        let res = bar
            .wrap_iter(self.links.iter().filter(|link| match link.method() {
                CLMethod::link => true,
                CLMethod::copy => false,
            }))
            .fold(Ok(()), |_, link_result| link_result.create_link());
        bar.finish();
        res
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum CLMethod {
    link,
    copy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLink {
    source: PathBuf,
    destination: PathBuf,
    method: CLMethod,
}

impl Linkable for ConfigLink {
    fn create_link(&self) -> Result<(), FailErr> {
        symlink::symlink_file(&self.source, &self.destination)
            .map_err(|_| InstallerErr::SymLinkFail)
            .map(Ok)?
    }

    fn method(&self) -> &CLMethod {
        &self.method
    }
}

trait Linkable {
    fn create_link(&self) -> Result<(), FailErr>;
    fn method(&self) -> &CLMethod;
}
