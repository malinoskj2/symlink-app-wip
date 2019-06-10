use std::fs;
use std::path::Path;

use crate::filters::filter_tags;
use crate::subcommand::{list::List, link::Link, SubCommand};
use crate::{option::Opt, types::*, FailErr};

pub fn init(opt: Opt) -> Result<(), FailErr> {
    info!("running init");

    match opt {
        Opt::Link(link) => link.exec().map(|_| ()),
        Opt::List(list) => list.exec(),
    }
}

pub fn handle_ok(res: &Linkfile<LinkData>) {
    info!("the file is ok");
    info!("parsed: {:#?}", res);
}

pub fn handle_err(err: &FailErr) {
    info!("the map has an error:\n{:#?}", err);
}
