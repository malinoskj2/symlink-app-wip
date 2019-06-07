extern crate env_logger;
extern crate lib_yamlink;
#[macro_use]
extern crate log;

use structopt::StructOpt;

use lib_yamlink::exec;
use lib_yamlink::option::Opt;
use lib_yamlink::FailErr;

fn main() -> Result<(), FailErr> {
    env_logger::init();
    debug!("op: \n{:#?}", Opt::from_args());

    exec::init(Opt::from_args())
        .map(|_| ())
        .map_err(|err: FailErr| err.into())
}
