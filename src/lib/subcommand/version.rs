use structopt::StructOpt;
use std::path::PathBuf;
use super::SubCommand;
use super::FailErr;
use structopt::clap::{crate_version, crate_name};

#[derive(StructOpt, Debug)]
pub struct Version {}

impl SubCommand for Version {
    fn exec(&self) -> Result<(), FailErr> {
        println!("{} {}", crate_name!(), crate_version!());
        Ok(())
    }
}
