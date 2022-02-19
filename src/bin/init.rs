use structopt::StructOpt;
use std::path::PathBuf;
use libgit_example::initialize_empty_repository;

#[derive(Debug, StructOpt)]
#[structopt(name = "init", about = "Example of Initializing Empty Repository")]
struct Opt {
    /// Path to the new repository
    repo: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    initialize_empty_repository(&opt.repo);
    println!("Empty repository created at {:?}!", opt.repo);
}
