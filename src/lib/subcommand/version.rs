use std::path::PathBuf;

use structopt::clap::{crate_name, crate_version};
use structopt::StructOpt;

use super::FailErr;
use super::SubCommand;

#[derive(StructOpt, Debug)]
pub struct Version {}

impl SubCommand for Version {
    fn exec(&self) -> Result<(), FailErr> {
        println!("{} {}", crate_name!(), crate_version!());
        Ok(())
    }
}
