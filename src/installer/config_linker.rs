use std::fs;
use std::path::{Path, PathBuf};

use crate::FailErr;

use super::error::InstallerErr;
use super::{fs_util, option::Opt};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use walkdir::DirEntry;

fn install<T: AsRef<str>, U: AsRef<Path>>(
    repo_path: U,
    cfg_names: &[T],
    sub_directories: &[U],
    tags: &[T],
) -> Result<(), FailErr> {
    let cfg_paths =
        fs_util::find_file_in_dir(repo_path.into(), cfg_names.into(), sub_directories.into())?;
    let cfg_paths: Vec<DirEntry> = cfg_paths;

    cfg_paths
        .into_iter()
        .flat_map(|entry| {
            let res: Result<ConfigMap<ConfigLink>, FailErr> = parse_config_map(entry.path());
            res
        })
        .filter(|cfg_map| apply_tag_filter(&cfg_map, tags))
        .map(|cfg_map| cfg_map.create_links())
        .collect::<Result<(), FailErr>>()
}

pub fn install_opts(opts: Opt) -> Result<(), FailErr> {
    let Opt {
        directories,
        config_names,
        sub_directories,
        tags,
        allow_privileged,
    } = opts;

    let res: Result<(), FailErr> = directories
        .into_iter()
        .map(|path| install(path, config_names.as_ref(), sub_directories.as_ref(), &tags))
        .collect();

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
    debug!("parsing:{:?} ", cfg_map.as_ref());
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

        let res =
            self.links.iter().filter(|link| match link.method() {
                CLMethod::link => true,
                CLMethod::copy => false,
            })
            .fold(Ok(()), |_, link_result| link_result.create_link());
        
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
        debug!("\nLinked: {:?} -> {:?}", &self.source, &self.destination);
        let link_res: Result<(), std::io::Error> =
            symlink::symlink_file(&self.source, &self.destination).map_err(|err| err.into());

        Ok(link_res?)
    }

    fn method(&self) -> &CLMethod {
        &self.method
    }
}

trait Linkable {
    fn create_link(&self) -> Result<(), FailErr>;
    fn method(&self) -> &CLMethod;
}
