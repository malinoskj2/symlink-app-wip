use std::env;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::filesystem::{find_config, parse, parse_linkfile};
use crate::types::*;
use crate::FailErr;

use super::SubCommand;
use std::fs::metadata;

#[derive(StructOpt, Debug)]
pub struct List {
    #[structopt(
    short = "n",
    long = "name",
    default_value = "[links.yaml, links.yml, links.toml]"
    )]
    pub config_names: Vec<String>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}

impl SubCommand for List {
    fn exec(&self) -> Result<String, FailErr> {
        let dir = env::current_dir()?;
        info!("running List");

        Ok(super::target_cfg_names(&self.config_names)
            .inspect(|name| debug!("target_config_name: {}", name))
            .filter_map(|config_name| find_config(&dir, &config_name))
            .flat_map(|config_path| parse_linkfile(config_path))
            .map(|linkfile| generate_output_string(&linkfile))
            .collect())

    }
}

fn generate_output_string(linkfile: &Linkfile<LinkData>) -> String {
    linkfile.get_link_metadata().iter()
        .map(|meta_res| {
            if let Ok(meta) = meta_res {
                print!("{}", meta);
                meta.to_string()
            } else {
                String::from("failed to get metadata")
            }
        }).collect()
}

fn log_linkfile_meta(linkfile: &Linkfile<LinkData>) {
    debug!("logging linkfile meta");

    linkfile.get_link_metadata().iter().for_each(|meta_res| {
        if let Ok(meta) = meta_res {
            print!("");
        } else {
            print!("failed to get metadata");
        }
    });
}
