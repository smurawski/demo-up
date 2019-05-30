use git2::Repository;
use std::path::PathBuf;

pub fn git_clone(url: &str, repo_path: PathBuf ) {
    let _repo =  match Repository::open(&repo_path) {
        Ok(repo) => {
            println!("\t\tRepository already exists.  Skipping.");
            repo
        },
        Err(_) => {
            match Repository::clone(url, repo_path) {
                Ok(repo) => repo,
                Err(e) => panic!("Failed to clone {}", e)
            }
        }
    };
}
