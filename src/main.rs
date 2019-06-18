extern crate env_logger;
extern crate lib_yamlink;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

use failure::ResultExt;
use exitfailure::ExitFailure;

use structopt::StructOpt;

use crate::lib_yamlink::subcommand::SubCommand;
use lib_yamlink::option::Opt;
use lib_yamlink::FailErr;
use std::fmt::Debug;

fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    debug!("op: \n{:#?}", Opt::from_args());

    Ok (Opt::from_args().exec()
        .map(|res_str|{
            print!("{}", res_str);
            ()
        })?)
}
