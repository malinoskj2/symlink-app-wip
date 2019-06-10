use std::fs;
use std::path::Path;

use crate::filters::filter_tags;
use crate::option::{Link, List};
use crate::{option::Opt, types::*, FailErr};

pub fn init(opt: Opt) -> Result<(), FailErr> {
    info!("running init");

    match opt {
        Opt::Link(link) => exec_link(link).map(|_| ()),
        Opt::List(list) => exec_list(list),
    }
}

fn exec_link(link: Link) -> Result<Vec<()>, FailErr> {
    let Link { config_files, tags } = link;

    let linkfiles = parse(config_files.as_ref(), tags.as_slice())?;

    let res: Result<Vec<()>, FailErr> = linkfiles
        .into_iter()
        .flat_map(|link_map| link_map.create_links())
        .collect();

    res
}

fn exec_list(list: List) -> Result<(), FailErr> {
    let List { config_files, tags } = list;

    let linkfiles = parse(config_files.as_ref(), tags.as_slice())?;

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

fn handle_ok(res: &Linkfile<LinkData>) {
    info!("the file is ok");
    info!("parsed: {:#?}", res);
}

fn handle_err(err: &FailErr) {
    info!("the map has an error:\n{:#?}", err);
}

fn parse_linkfile<U: AsRef<Path>>(cfg_map: U) -> Result<Linkfile<LinkData>, FailErr> {
    debug!("parsing:{:?} ", cfg_map.as_ref());
    let file = fs::File::open(&cfg_map)?;
    let cfg_map2 = serde_yaml::from_reader(file)?;
    Ok(cfg_map2)
}

fn parse(
    config_files: &[impl AsRef<Path>],
    tags: &[impl AsRef<str>],
) -> Result<Vec<Linkfile<LinkData>>, FailErr> {
    config_files
        .into_iter()
        .map(|path| parse_linkfile(path))
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
        .collect::<Result<Vec<Linkfile<LinkData>>, FailErr>>()
}
