use structopt::StructOpt;
use std::path::PathBuf;
use super::SubCommand;
use crate::FailErr;
use crate::filesystem::parse;

#[derive(StructOpt, Debug)]
pub struct List {
    #[structopt(short = "c", long = "config", default_value = "links.yml")]
    pub config_files: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    pub tags: Vec<String>,
}

impl SubCommand for List {
    fn exec(&self) -> Result<(), FailErr> {
        let linkfiles = parse(self.config_files.as_ref(),
                              self.tags.as_slice())?;

        linkfiles
            .iter()
            .flat_map(|file| (*file).get_link_metadata())
            .flatten()
            .inspect(|meta| {
                info!(
                    "\nstatus: {:#?}\nsource: {:#?}\nlinked @ {:#?}",
                    if meta.is_linked() { "Linked" } else { "Broken" },
                    meta.source(),
                    meta.destination()
                );
            });

        Ok(())
    }
}

