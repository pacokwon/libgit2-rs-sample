use libgit_example::{add_path, commit};
use git2::Repository;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "commit", about = "Example of commiting files to a given repository")]
struct Opt {
    /// path to git repository
    #[structopt(short, long)]
    repo: PathBuf,

    /// path to file to commit. must be relative to the repository
    #[structopt(short, long)]
    file: PathBuf,

    /// commit message
    #[structopt(short, long, default_value = "Commit from libgit2 binding")]
    message: String,
}

fn main() -> Result<(), git2::Error> {
    let opt = Opt::from_args();

    let repo = Repository::open(opt.repo)?;

    add_path(&repo, &opt.file)?;
    let oid = commit(&repo, &opt.message)?;

    println!("Successfully commited {:?}.\nCommit SHA - {:?}", opt.file, oid);

    Ok(())
}
