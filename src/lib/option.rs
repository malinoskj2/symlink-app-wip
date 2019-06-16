use std::path::PathBuf;
use structopt::StructOpt;
use crate::subcommand::list::List;
use crate::subcommand::link::Link;
use crate::subcommand::config::Config;
use crate::subcommand::version::Version;
use crate::subcommand::SubCommand;
use crate::FailErr;

#[derive(StructOpt, Debug)]
#[structopt(author = "")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub enum Opt {
    #[structopt(name = "link", about = "create symlinks", author = "")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Link(Link),
    #[structopt(name = "list", about = "print symlink state", author = "")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    List(List),
    #[structopt(name = "config", about = "validate and print config file", author = "")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Config(Config),
    #[structopt(name = "version", about = "show version information", author = "")]
    #[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Version(Version),
}

impl SubCommand for Opt {
    fn exec(&self) -> Result<(), FailErr> {
        match self {
            Opt::Link(link) => link.exec(),
            Opt::List(list) => list.exec(),
            Opt::Config(config) => config.exec(),
            Opt::Version(version) => version.exec(),
        }
    }
}
