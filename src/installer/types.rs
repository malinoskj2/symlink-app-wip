use std::path::{PathBuf, Path};
use crate::{option::Opt, FailErr};
use std::fs;
use std::ffi::OsString;

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
    destructive: bool
}

impl Default for CLOptions {
    fn default() -> Self {
        Self {
            destructive: true
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CLFilters {
    host: Option<String>,
    user: Option<String>,
}

impl CLFilters {
    fn filter_host(&self) -> bool {
        if self.host.as_ref().is_none() {
            true
        } else {
            hostname::get_hostname()
                .map_or_else(|| false, |host| self.host.as_ref().unwrap() == &host)
        }
    }

    fn filter_user(&self) -> bool {
        if self.user.as_ref().is_none() {
            true
        } else {
            users::get_current_username()
                .and_then(|user| user.into_string().ok())
                .map_or_else(|| false, |user| self.user.as_ref().unwrap() == &user)
        }
    }
}

impl Default for CLFilters {
    fn default() -> Self {
        Self {
            host: None,
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLink {
    source: PathBuf,
    destination: PathBuf,
    #[serde(default = "ConfigLink::method_default")]
    method: CLMethod,
    #[serde(default = "CLOptions::default")]
    options: CLOptions,
    #[serde(default = "CLFilters::default")]
    filters: CLFilters,
}

impl ConfigLink {
    fn method_default() -> CLMethod { CLMethod::Link }
}

impl ConfigLink {
    fn create_link(&self) -> Result<(), FailErr> {
        debug!("\nLinked: {:?} -> {:?}", &self.source, &self.destination);

        if self.options.destructive && self.destination.exists() {
            fs::remove_file(self.destination.as_path());
        }

        if self.filters.filter_host() && self.filters.filter_user() {
            let link_res: Result<(), std::io::Error> =
                symlink::symlink_file(&self.source, &self.destination)
                    .map_err(|err| err.into());
            Ok(link_res?)
        } else {
            Ok(())
        }
    }

    fn method(&self) -> &CLMethod {
        &self.method
    }
}
