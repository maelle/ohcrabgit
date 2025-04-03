use std::fs;
use std::fs::File;
use ohcrabgit::git;
use ohcrabgit::path;


pub fn exo(
    target: String
) -> String {

    let parent_path = path::create_target(target);
    let path = git::init_playground(&parent_path, "committed_to_wrong");

    // Create the Git mess :-)
    git::create_branch(&path, "feat-bla");


    git::create_branch(&path, "hot-fix");

    let fix = path.join("fix.txt");
    File::create(&fix).unwrap();
    fs::write(&fix, "hot\nfix").unwrap();
    git::add_file(&path, "fix.txt");
    git::commit(&path,"fix things");

    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();
    git::add_file(&path, "bla");
    git::commit(&path,"feat: add bla");

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