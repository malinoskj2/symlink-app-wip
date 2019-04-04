use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    #[structopt(name = "DIRECTORY", parse(from_os_str))]
    pub directories: Vec<PathBuf>,
    #[structopt(short = "c", long = "config-name", default_value = "link-map.yaml")]
    pub config_names: Vec<String>,
    #[structopt(short = "d", long = "specify-subdirs")]
    pub sub_directories: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
    #[structopt(short = "p", long = "allow-privileged")]
    pub allow_privileged: bool,
}
