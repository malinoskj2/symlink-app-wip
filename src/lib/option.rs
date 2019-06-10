use std::path::PathBuf;
use structopt::StructOpt;
use crate::subcommand::list::List;
use crate::subcommand::link::Link;

#[derive(StructOpt, Debug)]
#[structopt(name = "tbd", about = "make sym links easily.", author = "")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub enum Opt {
    #[structopt(name = "link", about = "create symlinks")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Link(Link),
    #[structopt(name = "list", about = "print symlink state")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    List(List),
}
