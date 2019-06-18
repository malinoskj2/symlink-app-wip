/// The enum holding the various errors used in the crate
use crate::types::{LinkData, Linkfile};
use crate::FailErr;
use std::fmt;

// internal errors
#[derive(Debug, Fail)]
pub enum InstallerErr {
    #[fail(display = "yaml parse error")]
    YamlParseFail(#[cause] serde_yaml::Error),
    #[fail(display = "yaml parse error IO")]
    IOYamlParseFail(#[cause] std::io::Error),
    #[fail(display = "Could not find config file")]
    NoConfigFile(#[cause] NoConfigFile),
}

impl From<serde_yaml::Error> for InstallerErr {
    fn from(err: serde_yaml::Error) -> Self {
        InstallerErr::YamlParseFail(err)
    }
}

impl From<std::io::Error> for InstallerErr {
    fn from(err: std::io::Error) -> Self {
        InstallerErr::IOYamlParseFail(err)
    }
}

impl From<NoConfigFile> for InstallerErr {
    fn from(err: NoConfigFile) -> Self {
        InstallerErr::NoConfigFile(err)
    }
}

pub fn handle_ok(res: &Linkfile<LinkData>) {
    info!("the file is ok");
    info!("parsed: {:#?}", res);
}

pub fn handle_err(err: &FailErr) {
    info!("the map has an error:\n{:#?}", err);
}


#[derive(Debug, Clone)]
pub struct NoConfigFile;

impl fmt::Display for NoConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find config file.")
    }
}

impl std::error::Error for NoConfigFile {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
