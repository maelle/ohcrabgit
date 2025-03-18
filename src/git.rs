use git2::Repository;
use git2::Signature;
use git2::Time;
use time::macros::{datetime};
use std::path::Path;

pub fn repo_init (
    path: &str
) {
    Repository::init(path).unwrap();
}

pub fn add_file(
    path: &str,
    file: &str,
) {
    let file = Path::new(file);
    let repo = Repository::open(path).expect("failed to open");
    let mut index = repo.index().expect("cannot get the Index file");
  
    index.add_path(file).unwrap();
    index.write().unwrap();
}

pub fn ohcrab_signature() -> Signature<'static> {
    let default_datetime = Time::new(
        datetime!(2023-12-15 15:25:00).assume_utc().unix_timestamp(), 
        0
    );
    
    Signature::new(
        "Jane Doe",
        "jane@example.com",
        &default_datetime
    ).unwrap()
}

pub fn first_commit(
    path: &str,
    message: &str
) {

    let repo = Repository::open(path).expect("failed to open");
    

    let signature = ohcrab_signature();
    
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[],
    ).expect("bug");
}

pub fn commit(
    path: &str,
    message: &str
) {
    let repo = Repository::open(path).expect("failed to open");

    let default_datetime = Time::new(
        datetime!(2023-12-15 15:25:00).assume_utc().unix_timestamp(), 
        0
    );
    let signature = Signature::new(
        "Jane Doe",
        "jane@example.com",
        &default_datetime
    ).expect("ouch");
    
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    
    let parent_commit = repo.head().expect("bug").peel_to_commit().unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit]
    ).expect("bug");
}