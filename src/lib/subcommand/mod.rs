use crate::filesystem;

use super::FailErr;

pub mod config;
pub mod link;
pub mod list;
pub mod version;

pub trait SubCommand {
    fn exec(&self) -> Result<(), FailErr>;
}

const DEFAULT_CFG_NAMES: [&str; 3] = ["links.yaml", "links.yml", "links.toml"];

// https://github.com/clap-rs/clap/issues/1452 need this if it gets impl'd
// If the user provides any config file names as arguments,
// concat the default names with those
pub fn target_cfg_names(from_args: &[impl AsRef<str>]) -> impl Iterator<Item = &str> {
    from_args
        .into_iter()
        .map(|name| name.as_ref())
        .chain(DEFAULT_CFG_NAMES.iter().map(|name| *name))
}
