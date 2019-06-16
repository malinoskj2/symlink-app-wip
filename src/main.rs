extern crate env_logger;
extern crate lib_yamlink;
#[macro_use]
extern crate log;

use structopt::StructOpt;

use lib_yamlink::option::Opt;
use lib_yamlink::FailErr;
use crate::lib_yamlink::subcommand::SubCommand;

fn main() -> Result<(), FailErr> {
    env_logger::init();
    debug!("op: \n{:#?}", Opt::from_args());

    Opt::from_args().exec()
        .map(|_| ())
        .map_err(|err: FailErr| err.into())
}
