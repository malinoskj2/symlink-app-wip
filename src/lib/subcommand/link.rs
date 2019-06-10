use structopt::StructOpt;
use std::path::PathBuf;
use super::SubCommand;
use super::FailErr;
use crate::filesystem::parse;

#[derive(StructOpt, Debug)]
pub struct Link {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}

impl SubCommand for Link {
    fn exec(&self) -> Result<(), FailErr> {

        let linkfiles = parse(self.config_files.as_ref(),
                              self.tags.as_slice())?;

        let res: Result<Vec<()>, FailErr> = linkfiles
            .into_iter()
            .flat_map(|link_map| link_map.create_links())
            .collect();

        Ok(())
    }
}
