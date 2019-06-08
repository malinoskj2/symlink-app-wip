use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tbd", about = "make sym links easily.", author = "")]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}
