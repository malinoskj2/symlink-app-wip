use std::path::PathBuf;
use structopt::StructOpt;

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

#[derive(StructOpt, Debug)]
pub struct Link {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub struct List {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}
