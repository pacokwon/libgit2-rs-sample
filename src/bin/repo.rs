use git2::Repository;

fn main() {
    let repo = match Repository::init("./dummy/repo") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to Init: {}", e),
    };

    println!("{:?}", repo.path());
}
