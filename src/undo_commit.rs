use std::fs;
use std::fs::File;
use ohcrabgit::git;
use ohcrabgit::path;


pub fn exo(
    target: String
) -> String {

    let parent_path = path::create_target(target);
    let path = git::init_playground(&parent_path, "undo_commit");

    // Create the Git mess :-)
    let fix = path.join("fix.txt");
    File::create(&fix).unwrap();
    fs::write(&fix, "lala").unwrap();
    git::add_file(&path, "fix.txt");
    git::commit(&path, "fix: fix things");

    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();
    git::add_file(&path, "bla");
    git::commit(&path,"add bla");

    fs::write(&bla, "thing1\nthing3").unwrap();
    git::add_file(&path, "bla");
    git::commit(&path,"edit bla");

    fs::write(&bla, "thing3\nthing3").unwrap();
    git::add_file(&path, "bla");
    git::commit(&path,"amend bla");

    println!("{}", path.display());
    return path.to_str().unwrap().to_string();
}

#[test]
fn it_works() {
    let dir = exo("tempdir".to_string());
    assert!(fs::exists(&dir).unwrap());

    let git_dir = dir + "/.git";
    assert!(fs::exists(&git_dir).unwrap());
}