extern crate lib_dot_installer;

use std::path::PathBuf;
use structopt::StructOpt;

use lib_dot_installer::FailErr;
use std::collections::HashMap;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "", parse(from_os_str))]
    paths: Vec<PathBuf>,
    #[structopt(short = "C", long = "config-name", default_value = "link-map.yaml")]
    config_name: Vec<String>,
    #[structopt(short = "D", long = "specify-dirs")]
    directories: Vec<String>,
    #[structopt(short = "H", long = "specify-hosts")]
    hosts: Vec<String>,
    #[structopt(short = "P", long = "allow-privileged")]
    allow_privileged: bool,
}

fn main() -> Result<(), FailErr> {
    let opt = Opt::from_args();

    println!("opt: {:?}", opt);

    Opt::from_args()
        .paths
        .into_iter()
        .for_each(|path: PathBuf| {
            // lib_dot_installer::install(path.as_path());
            //lib_dot_installer::package_installer::install(path.as_path());
        });

    Ok(())
}
