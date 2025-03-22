use std::fs;
use git2::Repository;
use git2::Signature;
use git2::Time;
use time::macros::{datetime};
use std::path::Path;
use std::path::PathBuf;

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

pub fn reset_hard(
    path: &str,
    ancestry: &str,
) {
    let repo = Repository::open(path).expect("failed to open");
    let object = repo.revparse_single(ancestry).unwrap();
    repo.reset(&object, git2::ResetType::Hard, None).unwrap();
}


pub fn init_playground(
    parent_path: &PathBuf,
    playground: &str,
)-> PathBuf {

    let folder_str = "exo-".to_string() + &playground;
    let path = parent_path.join(folder_str);
    let path_str =  &path.to_str().unwrap();
    fs::create_dir(&path).expect(&path_str);
    repo_init(&path_str);

    let gitignore = path.join(".gitignore");
    let _ = fs::copy("templates/gitignore.txt", &gitignore);
    add_file(&path_str, ".gitignore");
    first_commit(&path_str, "git ignore");

    let instructions = path.join("instructions.txt");
    let instructions_template = format!("templates/{}-instructions.txt", &playground.to_string());
    let _ = fs::copy(&instructions_template, &instructions);

    let tip = path.join("tip.txt");
    let tip_template = format!("templates/{}-tip.txt", &playground.to_string());
    let _ = fs::copy(&tip_template, &tip);

    return path;
}