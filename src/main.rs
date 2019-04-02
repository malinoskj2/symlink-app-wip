extern crate lib_yamlink;

use std::path::PathBuf;

use structopt::StructOpt;

use lib_yamlink::FailErr;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "DIRECTORY", parse(from_os_str))]
    directories: Vec<PathBuf>,
    #[structopt(short = "c", long = "config-name", default_value = "link-map.yaml")]
    config_names: Vec<String>,
    #[structopt(short = "d", long = "specify-subdirs")]
    sub_directories: Vec<PathBuf>,
    #[structopt(short = "t", long = "specify-tags")]
    tags: Vec<String>,
    #[structopt(short = "p", long = "allow-privileged")]
    allow_privileged: bool,
}

fn main() -> Result<(), FailErr> {
    let Opt {
        directories,
        config_names,
        sub_directories,
        tags,
        allow_privileged,
    } = Opt::from_args();

    directories.into_iter().for_each(|path| {
        lib_yamlink::config_linker::install(
            path,
            config_names.as_ref(),
            sub_directories.as_ref(),
            &tags,
        );
    });

    Ok(())
}
