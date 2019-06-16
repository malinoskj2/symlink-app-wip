pub mod link;
pub mod list;
pub mod config;
use super::FailErr;
use crate::filesystem;

pub trait SubCommand {
    fn exec(&self) -> Result<(), FailErr>;
}
