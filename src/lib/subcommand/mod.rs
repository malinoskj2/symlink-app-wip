use crate::filesystem;

use super::FailErr;
use std::fmt;

pub mod config;
pub mod link;
pub mod list;
pub mod version;

pub trait SubCommand {
    fn exec(&self) -> Result<String, FailErr>;
}

pub trait ErrDisplay {
    fn err_display(&self) -> String;
}

impl ErrDisplay for FailErr {
    fn err_display(&self) -> String {
        format!("Err Display")
    }
}

const DEFAULT_CFG_NAMES: [&str; 3] = ["links.yaml", "links.yml", "links.toml"];

// https://github.com/clap-rs/clap/issues/1452 need this if it gets impl'd
// If the user provides any config file names as arguments,
// concat the default names with those
pub fn target_cfg_names(from_args: &[impl AsRef<str>]) -> impl Iterator<Item = &str> {
    from_args
        .into_iter()
        .filter(|name|!DEFAULT_CFG_NAMES.iter().any(|def_name| *def_name == name.as_ref()) )
        .map(|name| name.as_ref())
        .chain(DEFAULT_CFG_NAMES.iter().map(|name| *name))
}
