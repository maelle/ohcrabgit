use std::fs;
use std::fs::File;
use ohcrabgit::git;
use ohcrabgit::path;


pub fn exo(
    target: String
) -> String {

    let parent_path = path::create_target(target);

    let path = git::init_playground(&parent_path, "time_machine");
    let path_str =  &path.to_str().unwrap();

    // Create the Git mess :-)
    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();

    git::add_file(&path_str, "bla");
    git::commit(&path_str,"feat: add bla");

    git::reset_hard(&path_str, "HEAD^");

    println!("{}", path.display());
    return path.to_str().unwrap().to_string();
}

#[test]
fn it_works() {
    let dir = exo("tempdir".to_string());
    println!("{}", dir);
    assert!(fs::exists(&dir).unwrap());

    let git_dir = dir + "/.git";
    assert!(fs::exists(&git_dir).unwrap());
}