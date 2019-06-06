extern crate lib_yamlink;
#[macro_use]
extern crate log;
extern crate env_logger;

use lib_yamlink::exec;
use lib_yamlink::option::Opt;
use lib_yamlink::FailErr;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), FailErr> {
    env_logger::init();
    info!("starting2");
    debug!("op: \n{:#?}", Opt::from_args());

    let Opt { config_files, tags } = Opt::from_args();

    exec::init(Opt::from_args())
        .map(|_| ())
        .map_err(|err: FailErr| err.into())
}
