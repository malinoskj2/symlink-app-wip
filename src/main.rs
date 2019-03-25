use std::fmt::Error;
use std::path::PathBuf;
use structopt::StructOpt;

type DIResult<T> = Result<T, Box<Error>>;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Paths to config repos
    #[structopt(name = "", parse(from_os_str))]
    paths: Vec<PathBuf>,
}

fn main() -> DIResult<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    Ok(())
}
