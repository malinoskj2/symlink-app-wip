extern crate lib_dot_installer;

use std::path::PathBuf;
use structopt::StructOpt;

use lib_dot_installer::{install, ConfigLink, DIResult};
use std::collections::HashMap;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "", parse(from_os_str))]
    paths: Vec<PathBuf>,
}

fn main() -> DIResult<()> {
    let config_map_paths: HashMap<String, ConfigLink> = Opt::from_args()
        .paths
        .into_iter()
        .flat_map(|path: PathBuf| install(path.as_path()))
        .collect();

    config_map_paths
        .iter()
        .for_each(|path| println!("Link: {:?}", path));

    Ok(())
}
