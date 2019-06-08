use std::fs;
use std::path::{Path, PathBuf};

use crate::filters::filter_tags;
use crate::{option::Opt, types::*, FailErr};
use crate::option::Link;

pub fn init<>(opt: Opt) -> Result<(), FailErr> {
    info!("running init");

    match opt {
        Opt::Link(link) => exec_link(link),
    }.map(|res|())

}

fn exec_link(link: Link) -> Result<Vec<()>, FailErr> {
    let Link {
        config_files,
        tags
    } = link;

    let res = config_files
        .into_iter()
        .map(|path| path.canonicalize())
        .map(|path| path.map(|path| parse_config_map2(path))?)
        .filter(|link_map| {
            if tags.is_empty() {
                true
            } else {
                link_map
                    .as_ref()
                    .ok()
                    .map_or_else(|| false, |res_map| filter_tags(&tags, &res_map))
            }
        })
        .inspect(|res| match res.as_ref() {
            Ok(res) => handle_ok(res),
            Err(err) => handle_err(err),
        })
        .collect::<Result<Vec<Linkfile<LinkData>>, FailErr>>();

    let res = res?;

    let res2: Result<Vec<()>, FailErr> = res
        .into_iter()
        .flat_map(|link_map| link_map.create_links())
        .collect();

    res2
}


fn handle_ok(res: &Linkfile<LinkData>) {
    info!("the file is ok");
    info!("parsed: {:#?}", res);
}

fn handle_err(err: &FailErr) {
    info!("the map has an error:\n{:#?}", err);
}

fn parse_config_map2<U: AsRef<Path>>(cfg_map: U) -> Result<Linkfile<LinkData>, FailErr> {
    debug!("parsing:{:?} ", cfg_map.as_ref());
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;
    Ok(cfg_map2)
}
