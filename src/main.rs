use std::fmt::Error;
use std::path::PathBuf;
use structopt::StructOpt;

type DIResult<T> = Result<T, Box<Error>>;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Files to process
    #[structopt(name = "", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> DIResult<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    Ok(())
}
