use std::path::{PathBuf, Path};
use crate::{option::Opt, FailErr};
use std::fs;

const DEFAULT_VEC_TAG_CAP: usize = 4;
const DEFAULT_VEC_LINK_CAP: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMap<ConfigLink> {
    tags: Vec<String>,
    links: Vec<ConfigLink>,
}

impl ConfigMap<ConfigLink> {
    pub fn contains_tag(&self, target_tag: &str) -> bool {
        self.tags.iter()
            .any(|tag| tag == target_tag)
    }

    pub fn create_links(&self) -> Vec<Result<(), FailErr>> {
        self.links.iter()
            .map(|link| link.create_link())
            .collect()
    }
}

impl Default for ConfigMap<ConfigLink> {
    fn default() -> Self {
        Self {
            tags: Vec::with_capacity(DEFAULT_VEC_TAG_CAP),
            links: Vec::with_capacity(DEFAULT_VEC_TAG_CAP),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum CLMethod {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "copy")]
    Copy,
}

#[derive(Debug, Serialize, Deserialize)]
struct CLOptions {
    #[serde(default = "CLOptions::destructive_default")]
    destructive: bool
}

impl CLOptions {
    fn destructive_default() -> bool { true }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLink {
    source: PathBuf,
    destination: PathBuf,
    #[serde(default = "ConfigLink::method_default")]
    method: CLMethod,
    options: CLOptions
}

impl ConfigLink {
    fn method_default() -> CLMethod { CLMethod::Link }
}


impl ConfigLink {
    fn create_link(&self) -> Result<(), FailErr> {
        debug!("\nLinked: {:?} -> {:?}", &self.source, &self.destination);

        if self.destination.exists() {
            fs::remove_file(self.destination.as_path());
        }

        let link_res: Result<(), std::io::Error> =
            symlink::symlink_file(&self.source, &self.destination).map_err(|err| err.into());
        Ok(link_res?)
    }

    fn method(&self) -> &CLMethod {
        &self.method
    }
}
