pub mod link;
pub mod list;
pub mod config;
pub mod version;
use super::FailErr;
use crate::filesystem;

pub trait SubCommand {
    fn exec(&self) -> Result<(), FailErr>;
}
