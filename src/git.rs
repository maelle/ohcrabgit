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


macro_rules! tmpl {
    ($name:literal, $kind:literal, $lang:literal) => {
        include_str!(concat!("../templates/", $name, "-", $kind, ".", $lang, ".txt"))
    };
}

fn get_template(name: &str, kind: &str, lang_code: &str) -> &'static str {
    match (name, kind, lang_code) {
        ("bisect", "instructions", "en") => tmpl!("bisect", "instructions", "en"),
        ("bisect", "instructions", "fr") => tmpl!("bisect", "instructions", "fr"),
        ("bisect", "instructions", "es") => tmpl!("bisect", "instructions", "es"),
        ("bisect", "tip", "en") => tmpl!("bisect", "tip", "en"),
        ("bisect", "tip", "fr") => tmpl!("bisect", "tip", "fr"),
        ("bisect", "tip", "es") => tmpl!("bisect", "tip", "es"),
        ("blame", "instructions", "en") => tmpl!("blame", "instructions", "en"),
        ("blame", "instructions", "fr") => tmpl!("blame", "instructions", "fr"),
        ("blame", "instructions", "es") => tmpl!("blame", "instructions", "es"),
        ("blame", "tip", "en") => tmpl!("blame", "tip", "en"),
        ("blame", "tip", "fr") => tmpl!("blame", "tip", "fr"),
        ("blame", "tip", "es") => tmpl!("blame", "tip", "es"),
        ("clean_dir", "instructions", "en") => tmpl!("clean_dir", "instructions", "en"),
        ("clean_dir", "instructions", "fr") => tmpl!("clean_dir", "instructions", "fr"),
        ("clean_dir", "instructions", "es") => tmpl!("clean_dir", "instructions", "es"),
        ("clean_dir", "tip", "en") => tmpl!("clean_dir", "tip", "en"),
        ("clean_dir", "tip", "fr") => tmpl!("clean_dir", "tip", "fr"),
        ("clean_dir", "tip", "es") => tmpl!("clean_dir", "tip", "es"),
        ("committed_to_main", "instructions", "en") => tmpl!("committed_to_main", "instructions", "en"),
        ("committed_to_main", "instructions", "fr") => tmpl!("committed_to_main", "instructions", "fr"),
        ("committed_to_main", "instructions", "es") => tmpl!("committed_to_main", "instructions", "es"),
        ("committed_to_main", "tip", "en") => tmpl!("committed_to_main", "tip", "en"),
        ("committed_to_main", "tip", "fr") => tmpl!("committed_to_main", "tip", "fr"),
        ("committed_to_main", "tip", "es") => tmpl!("committed_to_main", "tip", "es"),
        ("committed_to_wrong", "instructions", "en") => tmpl!("committed_to_wrong", "instructions", "en"),
        ("committed_to_wrong", "instructions", "fr") => tmpl!("committed_to_wrong", "instructions", "fr"),
        ("committed_to_wrong", "instructions", "es") => tmpl!("committed_to_wrong", "instructions", "es"),
        ("committed_to_wrong", "tip", "en") => tmpl!("committed_to_wrong", "tip", "en"),
        ("committed_to_wrong", "tip", "fr") => tmpl!("committed_to_wrong", "tip", "fr"),
        ("committed_to_wrong", "tip", "es") => tmpl!("committed_to_wrong", "tip", "es"),
        ("conflict", "instructions", "en") => tmpl!("conflict", "instructions", "en"),
        ("conflict", "instructions", "fr") => tmpl!("conflict", "instructions", "fr"),
        ("conflict", "instructions", "es") => tmpl!("conflict", "instructions", "es"),
        ("conflict", "tip", "en") => tmpl!("conflict", "tip", "en"),
        ("conflict", "tip", "fr") => tmpl!("conflict", "tip", "fr"),
        ("conflict", "tip", "es") => tmpl!("conflict", "tip", "es"),
        ("latest_message", "instructions", "en") => tmpl!("latest_message", "instructions", "en"),
        ("latest_message", "instructions", "fr") => tmpl!("latest_message", "instructions", "fr"),
        ("latest_message", "instructions", "es") => tmpl!("latest_message", "instructions", "es"),
        ("latest_message", "tip", "en") => tmpl!("latest_message", "tip", "en"),
        ("latest_message", "tip", "fr") => tmpl!("latest_message", "tip", "fr"),
        ("latest_message", "tip", "es") => tmpl!("latest_message", "tip", "es"),
        ("log_deleted_file", "instructions", "en") => tmpl!("log_deleted_file", "instructions", "en"),
        ("log_deleted_file", "instructions", "fr") => tmpl!("log_deleted_file", "instructions", "fr"),
        ("log_deleted_file", "instructions", "es") => tmpl!("log_deleted_file", "instructions", "es"),
        ("log_deleted_file", "tip", "en") => tmpl!("log_deleted_file", "tip", "en"),
        ("log_deleted_file", "tip", "fr") => tmpl!("log_deleted_file", "tip", "fr"),
        ("log_deleted_file", "tip", "es") => tmpl!("log_deleted_file", "tip", "es"),
        ("log_deleted_line", "instructions", "en") => tmpl!("log_deleted_line", "instructions", "en"),
        ("log_deleted_line", "instructions", "fr") => tmpl!("log_deleted_line", "instructions", "fr"),
        ("log_deleted_line", "instructions", "es") => tmpl!("log_deleted_line", "instructions", "es"),
        ("log_deleted_line", "tip", "en") => tmpl!("log_deleted_line", "tip", "en"),
        ("log_deleted_line", "tip", "fr") => tmpl!("log_deleted_line", "tip", "fr"),
        ("log_deleted_line", "tip", "es") => tmpl!("log_deleted_line", "tip", "es"),
        ("rebase_i", "instructions", "en") => tmpl!("rebase_i", "instructions", "en"),
        ("rebase_i", "instructions", "fr") => tmpl!("rebase_i", "instructions", "fr"),
        ("rebase_i", "instructions", "es") => tmpl!("rebase_i", "instructions", "es"),
        ("rebase_i", "tip", "en") => tmpl!("rebase_i", "tip", "en"),
        ("rebase_i", "tip", "fr") => tmpl!("rebase_i", "tip", "fr"),
        ("rebase_i", "tip", "es") => tmpl!("rebase_i", "tip", "es"),
        ("reset", "instructions", "en") => tmpl!("reset", "instructions", "en"),
        ("reset", "instructions", "fr") => tmpl!("reset", "instructions", "fr"),
        ("reset", "instructions", "es") => tmpl!("reset", "instructions", "es"),
        ("reset", "tip", "en") => tmpl!("reset", "tip", "en"),
        ("reset", "tip", "fr") => tmpl!("reset", "tip", "fr"),
        ("reset", "tip", "es") => tmpl!("reset", "tip", "es"),
        ("revparse", "instructions", "en") => tmpl!("revparse", "instructions", "en"),
        ("revparse", "instructions", "fr") => tmpl!("revparse", "instructions", "fr"),
        ("revparse", "instructions", "es") => tmpl!("revparse", "instructions", "es"),
        ("revparse", "tip", "en") => tmpl!("revparse", "tip", "en"),
        ("revparse", "tip", "fr") => tmpl!("revparse", "tip", "fr"),
        ("revparse", "tip", "es") => tmpl!("revparse", "tip", "es"),
        ("small_change", "instructions", "en") => tmpl!("small_change", "instructions", "en"),
        ("small_change", "instructions", "fr") => tmpl!("small_change", "instructions", "fr"),
        ("small_change", "instructions", "es") => tmpl!("small_change", "instructions", "es"),
        ("small_change", "tip", "en") => tmpl!("small_change", "tip", "en"),
        ("small_change", "tip", "fr") => tmpl!("small_change", "tip", "fr"),
        ("small_change", "tip", "es") => tmpl!("small_change", "tip", "es"),
        ("split_changes", "instructions", "en") => tmpl!("split_changes", "instructions", "en"),
        ("split_changes", "instructions", "fr") => tmpl!("split_changes", "instructions", "fr"),
        ("split_changes", "instructions", "es") => tmpl!("split_changes", "instructions", "es"),
        ("split_changes", "tip", "en") => tmpl!("split_changes", "tip", "en"),
        ("split_changes", "tip", "fr") => tmpl!("split_changes", "tip", "fr"),
        ("split_changes", "tip", "es") => tmpl!("split_changes", "tip", "es"),
        ("time_machine", "instructions", "en") => tmpl!("time_machine", "instructions", "en"),
        ("time_machine", "instructions", "fr") => tmpl!("time_machine", "instructions", "fr"),
        ("time_machine", "instructions", "es") => tmpl!("time_machine", "instructions", "es"),
        ("time_machine", "tip", "en") => tmpl!("time_machine", "tip", "en"),
        ("time_machine", "tip", "fr") => tmpl!("time_machine", "tip", "fr"),
        ("time_machine", "tip", "es") => tmpl!("time_machine", "tip", "es"),
        ("undo_commit", "instructions", "en") => tmpl!("undo_commit", "instructions", "en"),
        ("undo_commit", "instructions", "fr") => tmpl!("undo_commit", "instructions", "fr"),
        ("undo_commit", "instructions", "es") => tmpl!("undo_commit", "instructions", "es"),
        ("undo_commit", "tip", "en") => tmpl!("undo_commit", "tip", "en"),
        ("undo_commit", "tip", "fr") => tmpl!("undo_commit", "tip", "fr"),
        ("undo_commit", "tip", "es") => tmpl!("undo_commit", "tip", "es"),
        ("undo_file", "instructions", "en") => tmpl!("undo_file", "instructions", "en"),
        ("undo_file", "instructions", "fr") => tmpl!("undo_file", "instructions", "fr"),
        ("undo_file", "instructions", "es") => tmpl!("undo_file", "instructions", "es"),
        ("undo_file", "tip", "en") => tmpl!("undo_file", "tip", "en"),
        ("undo_file", "tip", "fr") => tmpl!("undo_file", "tip", "fr"),
        ("undo_file", "tip", "es") => tmpl!("undo_file", "tip", "es"),
        ("worktree", "instructions", "en") => tmpl!("worktree", "instructions", "en"),
        ("worktree", "instructions", "fr") => tmpl!("worktree", "instructions", "fr"),
        ("worktree", "instructions", "es") => tmpl!("worktree", "instructions", "es"),
        ("worktree", "tip", "en") => tmpl!("worktree", "tip", "en"),
        ("worktree", "tip", "fr") => tmpl!("worktree", "tip", "fr"),
        ("worktree", "tip", "es") => tmpl!("worktree", "tip", "es"),
        _ => "",
    }
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
    fs::write(&gitignore, include_str!("../templates/gitignore.txt")).unwrap();
    add_file(&path, ".gitignore");
    first_commit(&path, "git ignore");

    let instructions = path.join("instructions.txt");
    fs::write(&instructions, get_template(playground, "instructions", lang.code())).unwrap();

    let tip = path.join("tip.txt");
    fs::write(&tip, get_template(playground, "tip", lang.code())).unwrap();

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