use std::fs;
use std::path::{Path, PathBuf};

use crate::FailErr;
use std::convert::TryFrom;
use std::fs::Metadata;

const DEFAULT_VEC_TAG_CAP: usize = 4;
const DEFAULT_VEC_LINK_CAP: usize = 32;

// LinkFile
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Linkfile<T> {
    tags: Vec<String>,
    links: Vec<T>,
}

impl Linkfile<LinkData> {
    pub fn contains_tag(&self, target_tag: &str) -> bool {
        self.tags.iter().any(|tag| tag == target_tag)
    }

    pub fn create_links(&self) -> Vec<Result<(), FailErr>> {
        self.links.iter().map(|link| link.create_link()).collect()
    }

    pub fn get_link_metadata<'a>(&self) -> Vec<Result<LinkMeta, std::io::Error>> {
        self.links.iter().map(LinkMeta::try_from).collect()
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

// LinkMethod
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkMethod {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "copy")]
    Copy,
}

// LinkOptions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkOptions {
    destructive: bool,
}

impl Default for LinkOptions {
    fn default() -> Self {
        Self { destructive: true }
    }
}

// LinkConditions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkConditions {
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

// LinkData
#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
#[builder(setter(into))]
pub struct LinkData {
    pub source: PathBuf,
    pub destination: PathBuf,
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

impl Default for LinkData {
    fn default() -> Self {
        Self {
            source: Default::default(),
            destination: Default::default(),
            method: LinkMethod::Link,
            options: LinkOptions::default(),
            filters: LinkConditions::default(),
        }
    }
}

impl LinkData {
    pub fn create_link(&self) -> Result<(), FailErr> {
        debug!("\nLinked: {:?} -> {:?}", &self.source, &self.destination);

        if self.options.destructive && self.destination.exists() {
            fs::remove_file(self.destination.as_path()).expect("Failed to remove file");
        }

        if self.should_create() {
            if let Some(parent) = self.destination_dir_if_exists() {
                std::fs::create_dir_all(parent);
            }

            symlink::symlink_file(&self.source, &self.destination).map_err(|err| err.into())
        } else {
            Ok(())
        }
    }

    fn method(&self) -> &LinkMethod {
        &self.method
    }

    fn should_create(&self) -> bool {
        self.filters.filter_host() && self.filters.filter_user()
    }

    // returns path to parent directory if it exists, otherwise None
    pub fn destination_dir_if_exists(&self) -> Option<&Path> {
        if let Some(parent) = self.destination.parent() {
            if parent.exists() {
                Some(parent)
            } else {
                None
            }
        } else {
            if self.destination.has_root() {
                Some(self.destination.as_path())
            } else {
                None
            }
        }
    }

    pub fn destination_dir_exists(&self) -> bool {
        self.destination_dir_if_exists().is_some()
    }
}

impl TryFrom<&LinkData> for Metadata {
    type Error = std::io::Error;
    fn try_from(link: &LinkData) -> Result<Self, Self::Error> {
        link.destination.symlink_metadata()
    }
}

// LinkMeta
pub struct LinkMeta<'a> {
    source: &'a Path,
    destination: &'a Path,
    meta: std::fs::Metadata,
}

impl<'a> LinkMeta<'a> {
    pub fn is_linked(&self) -> bool {
        self.meta.file_type().is_symlink()
    }
    pub fn source(&self) -> &Path {
        self.source
    }
    pub fn destination(&self) -> &Path {
        self.destination
    }
}

impl<'a> TryFrom<&'a LinkData> for LinkMeta<'a> {
    type Error = std::io::Error;

    fn try_from(link: &'a LinkData) -> Result<Self, Self::Error> {
        std::fs::Metadata::try_from(link).map(|res| LinkMeta {
            source: link.source.as_path(),
            destination: link.destination.as_path(),
            meta: res,
        })
    }
}
