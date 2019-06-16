use structopt::StructOpt;
use std::path::{PathBuf, Path};
use std::{env, fs};
use super::SubCommand;
use crate::FailErr;
use crate::filesystem::{parse, parse_linkfile};
use crate::types::{Linkfile, LinkData};
use walkdir::WalkDir;
use core::borrow::Borrow;

#[derive(StructOpt, Debug)]
pub struct Config {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<String>
}

impl SubCommand for Config {
    fn exec(&self) -> Result<(), FailErr> {
        let dir = env::current_dir()?;

        self.config_files
            .iter()
            .filter_map(|config_name| find_config(&dir, &config_name))
            .map(|path| fs::read_to_string(path))
            .for_each(|cfg_str_res| {
                if let Ok(cfg_str) = cfg_str_res {
                    if serde_yaml::from_str::<Linkfile<LinkData>>(&cfg_str).is_ok() {
                        println!("{}", cfg_str);
                    } else {
                        println!("config is invalid, failed to parse");
                    }
                } else {
                    println!("Failed to read config");
                }
            });

        Ok(())
    }
}

fn find_config(dir: impl AsRef<Path>, config_name: &str) -> Option<PathBuf> {
    WalkDir::new(dir.as_ref())
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| {
            if let Some(file_name) = e.file_name().to_str() {
                if file_name == config_name { true } else { false }
            } else {
                false
            }
        })
        .map(|e| e.into_path())
}
