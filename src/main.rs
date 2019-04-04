extern crate lib_yamlink;
#[macro_use]
extern crate log;
extern crate env_logger;

use lib_yamlink::config_linker::install_opts;
use lib_yamlink::option::Opt;
use lib_yamlink::FailErr;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), FailErr> {
    env_logger::init();

    info!("starting");
    debug!("op: \n{:#?}", Opt::from_args());

    install_opts(Opt::from_args())
        .map(|_| ())
        .map_err(|err| err.into())
}
