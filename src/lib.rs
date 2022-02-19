use std::path::Path;
use git2::{Oid, Signature, Repository};

pub fn initialize_empty_repository<P>(path: P) -> Repository
    where P: AsRef<Path>
{
    match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("{:?}", e),
    }
}
