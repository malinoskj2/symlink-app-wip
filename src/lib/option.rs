use std::path::PathBuf;
use structopt::StructOpt;
use crate::subcommand::list::List;
use crate::subcommand::link::Link;
use crate::subcommand::config::Config;
use crate::subcommand::SubCommand;
use crate::FailErr;

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
    #[structopt(name = "config", about = "validate and print config file")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Config(Config),
}

impl SubCommand for Opt {
    fn exec(&self) -> Result<(), FailErr> {
        match self {
            Opt::Link(link) => link.exec(),
            Opt::List(list) => list.exec(),
            Opt::Config(config) => config.exec(),
        }
    }
}
