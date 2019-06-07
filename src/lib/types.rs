use std::fs;
use std::path::PathBuf;

use crate::FailErr;

const DEFAULT_VEC_TAG_CAP: usize = 4;
const DEFAULT_VEC_LINK_CAP: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
pub struct Linkfile<ConfigLink> {
    tags: Vec<String>,
    links: Vec<ConfigLink>,
}

impl Linkfile<LinkData> {
    pub fn contains_tag(&self, target_tag: &str) -> bool {
        self.tags.iter().any(|tag| tag == target_tag)
    }

    pub fn create_links(&self) -> Vec<Result<(), FailErr>> {
        self.links.iter().map(|link| link.create_link()).collect()
    }
}

impl Default for Linkfile<LinkData> {
    fn default() -> Self {
        Self {
            tags: Vec::with_capacity(DEFAULT_VEC_TAG_CAP),
            links: Vec::with_capacity(DEFAULT_VEC_LINK_CAP),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum LinkMethod {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "copy")]
    Copy,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinkOptions {
    destructive: bool,
}

impl Default for LinkOptions {
    fn default() -> Self {
        Self { destructive: true }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LinkConditions {
    host: Option<String>,
    user: Option<String>,
}

impl LinkConditions {
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

impl Default for LinkConditions {
    fn default() -> Self {
        Self {
            host: None,
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkData {
    source: PathBuf,
    destination: PathBuf,
    #[serde(default = "LinkData::method_default")]
    method: LinkMethod,
    #[serde(default = "LinkOptions::default")]
    options: LinkOptions,
    #[serde(default = "LinkConditions::default")]
    filters: LinkConditions,
}

impl LinkData {
    fn method_default() -> LinkMethod {
        LinkMethod::Link
    }
}

impl LinkData {
    fn create_link(&self) -> Result<(), FailErr> {
        debug!("\nLinked: {:?} -> {:?}", &self.source, &self.destination);

        if self.options.destructive && self.destination.exists() {
            fs::remove_file(self.destination.as_path())
                .expect("Failed to remove file");
        }

        if self.filters.filter_host() && self.filters.filter_user() {
            let link_res: Result<(), std::io::Error> =
                symlink::symlink_file(&self.source, &self.destination).map_err(|err| err.into());
            Ok(link_res?)
        } else {
            Ok(())
        }
    }

    fn method(&self) -> &LinkMethod {
        &self.method
    }
}
