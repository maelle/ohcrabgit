use std::fs;
use git2::Repository;
use git2::Signature;
use git2::Time;
use time::macros::{datetime};
use std::path::Path;
use std::path::PathBuf;
use crate::lang::Lang;

pub fn repo_init (
    path: &PathBuf
) {
    Repository::init(path).unwrap();
}

pub fn add_file(
    path: &PathBuf,
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
    path: &PathBuf,
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
    path: &PathBuf,
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
    path: &PathBuf,
    ancestry: &str,
) {
    let repo = Repository::open(path).expect("failed to open");
    let object = repo.revparse_single(ancestry).unwrap();
    repo.reset(&object, git2::ResetType::Hard, None).unwrap();
}


pub fn init_playground(
    parent_path: &PathBuf,
    playground: &str,
    lang: &Lang,
) -> PathBuf {

    let folder_str = "exo-".to_string() + &playground;
    let path = parent_path.join(folder_str);
    let path_str =  &path.to_str().unwrap();
    if fs::exists(&path).unwrap() {
        eprintln!("Exercise folder already exists: {} — delete it or choose a different target.", path_str);
        std::process::exit(1);
    }
    fs::create_dir(&path).unwrap();
    repo_init(&path);

    let gitignore = path.join(".gitignore");
    let _ = fs::copy("templates/gitignore.txt", &gitignore);
    add_file(&path, ".gitignore");
    first_commit(&path, "git ignore");

    let instructions = path.join("instructions.txt");
    let instructions_template = format!("templates/{}-instructions.{}.txt", &playground, lang.code());
    let _ = fs::copy(&instructions_template, &instructions);

    let tip = path.join("tip.txt");
    let tip_template = format!("templates/{}-tip.{}.txt", &playground, lang.code());
    let _ = fs::copy(&tip_template, &tip);

    return path;
}

pub fn checkout_branch(
    path: &PathBuf,
    branch_name: &str,
) {
    let repo = Repository::open(path).expect("failed to open");
    let (object, reference) = repo.revparse_ext(branch_name).unwrap();
    repo.checkout_tree(&object, None).unwrap();
    repo.set_head(reference.unwrap().name().unwrap()).expect("Failed to set HEAD");
}

pub fn current_branch(
    path: &PathBuf,
) -> String {
    let repo = Repository::open(path).expect("failed to open");
    let head = repo.head().unwrap();
    head.shorthand().unwrap().to_string()
}

pub fn remove_file(
    path: &PathBuf,
    filename: &str,
) {
    fs::remove_file(path.join(filename)).unwrap();
    let file = Path::new(filename);
    let repo = Repository::open(path).expect("failed to open");
    let mut index = repo.index().expect("cannot get the Index file");
    index.remove_path(file).unwrap();
    index.write().unwrap();
}

pub fn commit_with_author(
    path: &PathBuf,
    message: &str,
    name: &str,
    email: &str,
    unix_ts: i64,
) {
    let repo = Repository::open(path).expect("failed to open");

    let author_time = Time::new(unix_ts, 0);
    let author = Signature::new(name, email, &author_time).unwrap();

    let committer_time = Time::new(
        datetime!(2023-12-15 15:25:00).assume_utc().unix_timestamp(),
        0,
    );
    let committer = Signature::new("Jane Doe", "jane@example.com", &committer_time).unwrap();

    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let parent_commit = repo.head().expect("bug").peel_to_commit().unwrap();

    repo.commit(
        Some("HEAD"),
        &author,
        &committer,
        message,
        &tree,
        &[&parent_commit],
    ).expect("bug");
}

pub fn create_tag(
    path: &PathBuf,
    tag_name: &str,
    ancestry: &str,
) {
    let repo = Repository::open(path).expect("failed to open");
    let object = repo.revparse_single(ancestry).unwrap();
    repo.tag_lightweight(tag_name, &object, false).unwrap();
}

pub fn create_branch(
    path: &PathBuf,
    branch_name: &str
) {
    let repo = Repository::open(path).expect("failed to open");
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    repo.branch(
    &branch_name,
    &parent_commit,
    true,
    ).unwrap();

    // https://stackoverflow.com/a/67240436
    let  (object, reference) = repo.revparse_ext(branch_name).unwrap();
    repo.checkout_tree(&object, None).unwrap();
    repo.set_head(&reference.unwrap().name().unwrap()).expect("Failed to set HEAD");

}