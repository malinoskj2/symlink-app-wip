use super::error::InstallerErr;
use super::filters;
use crate::{option::Opt, FailErr, types::*};
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;
use std::fs;
use std::error::Error;
use crate::filters::filter_tags;

pub fn init(opt: Opt) -> Result<(), FailErr> {
    info!("running init");

    let Opt {
        config_files,
        tags
    } = opt;

    let res = config_files
        .into_iter()
        .map(|path| path.canonicalize())
        .map(|path| {
            path.map(|path| parse_config_map2(path))?
        })
        .filter(|link_map| {
            if tags.is_empty() { true } else {
                link_map.as_ref().ok()
                    .map_or_else(|| false, |res_map| filter_tags(&tags, &res_map))
            }
        })
        .inspect(|res| match res.as_ref().is_ok() {
            true => handle_ok(res.as_ref().unwrap()),
            false => handle_err(res.as_ref().unwrap_err())
        })
        .collect::<Result<Vec<ConfigMap<ConfigLink>>, FailErr>>();

    let res = res?;
    let res2: Vec<Result<(), FailErr>> = res.into_iter()
        .flat_map(|link_map| link_map.create_links())
        .collect();

    Ok(())
}

fn handle_ok(res: &ConfigMap<ConfigLink>) {
    info!("the map is ok");
    info!("parsed: {:#?}", res);
}

fn handle_err(err: &FailErr) {
    info!("the map has an error:\n{:#?}", err);
}

fn parse_config_map<U: AsRef<Path>>(
    cfg_map: U,
) -> Result<ConfigMap<ConfigLink>, FailErr> {
    debug!("parsing:{:?} ", cfg_map.as_ref());
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;
    Ok(cfg_map2)
}

fn parse_config_map2<U: AsRef<Path>>(
    cfg_map: U,
) -> Result<ConfigMap<ConfigLink>, FailErr> {
    debug!("parsing:{:?} ", cfg_map.as_ref());
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;
    Ok(cfg_map2)
}
