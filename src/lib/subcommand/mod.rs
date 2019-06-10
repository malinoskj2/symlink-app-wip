pub mod link;
pub mod list;
use super::FailErr;
use crate::filesystem;

pub trait SubCommand {
    fn exec(&self) -> Result<(), FailErr>;
}
