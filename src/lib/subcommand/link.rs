use std::path::PathBuf;

use structopt::StructOpt;

use crate::filesystem;
use crate::types::Linkfile;

use super::FailErr;
use super::SubCommand;
use std::env;
use crate::filesystem::parse_linkfile;

#[derive(StructOpt, Debug)]
pub struct Link {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<String>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}

impl SubCommand for Link {
    fn exec(&self) -> Result<String, FailErr> {
        // Result<Vec<()>, FailErr>
        let dir = env::current_dir()?;

        let res: Vec<Result<(), FailErr>> = super::target_cfg_names(&self.config_files)
            .flat_map(|config_name| filesystem::find_config(&dir, &config_name))
            .flat_map(|cfg_path| parse_linkfile(cfg_path))
            .map(|linkfile| linkfile.create_links())
            .flatten()
            .collect();

        Ok(String::from("linked okay"))
    }
}
