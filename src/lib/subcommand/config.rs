use core::borrow::Borrow;
use std::path::{Path, PathBuf};
use std::{env, fs};

use structopt::StructOpt;
use walkdir::WalkDir;

use crate::filesystem::{find_config, parse, parse_linkfile};
use crate::types::{LinkData, Linkfile};
use crate::FailErr;
use crate::error::NoConfigFile;

use super::SubCommand;

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(
        short = "n",
        long = "name",
        default_value = "[links.yaml, links.yml, links.toml]"
    )]
    pub config_names: Vec<String>,
}

impl SubCommand for Config {
    fn exec(&self) -> Result<String, FailErr> {
        let dir = env::current_dir()?;

        let config_strings = super::target_cfg_names(&self.config_names)
            .flat_map(|config_name| find_config(&dir, &config_name))
            .flat_map(fs::read_to_string)
            .collect::<Vec<String>>();

        if config_strings.is_empty() {
            Err(NoConfigFile)?
        } else {
            Ok(config_strings.join("\n"))
        }
    }
}



fn print_res(output: Result<String, std::io::Error>) {
    if let Ok(cfg_str) = output {
        if serde_yaml::from_str::<Linkfile<LinkData>>(&cfg_str).is_ok() {
            println!("{}", cfg_str);
        } else {
            println!("config is invalid, failed to parse");
        }
    } else {
        println!("Failed to read config");
    }
}
