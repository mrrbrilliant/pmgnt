use git2::Repository;

pub fn git_clone(url: &str, clone_to: &str) {

    match Repository::clone(url, clone_to) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}
