use std::path::PathBuf;
use std::process;
use git2::{Commit, ObjectType, Repository, Oid, IndexAddOption};
use git2;

pub fn initrepo(dir: &PathBuf) {
    match Repository::init(dir) {
        Ok(_repo) => (),
        Err(e) => {
            println!("Error: Failed to init: {}", e);
            process::exit(1);
        }
    };
}

pub fn commitall(dir: &PathBuf) -> Result<(), git2::Error> {
     let repo = match Repository::open(dir) {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };

    match self::add_and_commit(&repo, "Updated dotfiles") {
        Ok(_oid) => (),
        Err(e) => {
            println!("Error: Failed to commit: {}", e);
            process::exit(1);
        }
    };

    return Ok(());
}

fn add_and_commit(repo: &Repository, message: &str) -> Result<Oid, git2::Error> {
    let mut firstcommit = false;
    let mut index = repo.index()?;
    index.add_all(
        ["*"].iter(),
        IndexAddOption::DEFAULT,
        None,
    )?;
    let oid = index.write_tree()?;
    index.write()?;
    let signature = repo.signature()?;
    let mut parent_commit: Option<Commit> = None;
    match find_last_commit(repo) {
        Ok(commit) => {
            parent_commit = Some(commit);
        }
        Err(_) => {
            firstcommit = true;
        }
    }
    let tree = repo.find_tree(oid)?;
    if firstcommit {
        repo.commit(
            Some("HEAD"), //  point HEAD to our new commit
            &signature, // author
            &signature, // committer
            message, // commit message
            &tree, // tree
            &[], // parents
        )
    } else {
        repo.commit(
            Some("HEAD"), //  point HEAD to our new commit
            &signature, // author
            &signature, // committer
            message, // commit message
            &tree, // tree
            &[&parent_commit.unwrap()], // parents
        )
    }
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?;
    let obj = obj.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
