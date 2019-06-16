use std::path::Path;
use crate::types::LinkData;
use std::fs;
use crate::types::Linkfile;
use crate::FailErr;
use crate::error;

fn parse_linkfile<U: AsRef<Path>>(cfg_map: U) -> Result<Linkfile<LinkData>, FailErr> {
    debug!("parsing:{:?} ", cfg_map.as_ref());
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;
    Ok(cfg_map2)
}

pub fn parse(
    config_files: &[impl AsRef<Path>],
    tags: &[impl AsRef<str>],
) -> Result<Vec<Linkfile<LinkData>>, FailErr> {
    config_files
        .into_iter()
        .map(|path| parse_linkfile(path))
        .inspect(|res| match res.as_ref() {
            Ok(res) => error::handle_ok(res),
            Err(err) => error::handle_err(err),
        })
        .collect::<Result<Vec<Linkfile<LinkData>>, FailErr>>()
}
