use std::path::Path;
use git2::{Oid, Repository};

pub fn initialize_empty_repository<P>(path: P) -> Repository
    where P: AsRef<Path>
{
    match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("{:?}", e),
    }
}

pub fn last_commit(repo: &Repository) -> Result<Option<git2::Commit>, git2::Error> {
    match repo.head() {
        Ok(rf) => Ok(Some(rf.peel_to_commit()?)),
        Err(_) => Ok(None),
    }
}

pub fn add_path(repo: &Repository, path: &Path) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    index.write()?;
    Ok(())
}

pub fn commit(repo: &Repository, message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let signature = repo.signature()?;

    let parent_commit;
    let parent = match last_commit(repo)? {
        Some(c) => {
            parent_commit = c;
            vec![&parent_commit]
        },
        _ => vec![],
    };

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parent,
    )
}
