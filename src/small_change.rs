use std::fs;
use std::fs::File;
use ohcrabgit::git;
use ohcrabgit::path;


pub fn exo(
    target: String
) -> String {

    let parent_path = path::create_target(target);

    let path = parent_path.join("exo-small_change");
    let path_str =  &path.to_str().unwrap();
    fs::create_dir(&path).expect(&path_str);
    git::repo_init(&path_str);

    let gitignore = path.join(".gitignore");
    let _ = fs::copy("templates/gitignore.txt", &gitignore);
    git::add_file(&path_str, ".gitignore");
    git::first_commit(&path_str, "git ignore");

    let instructions = path.join("instructions.txt");
    let _ = fs::copy("templates/small_change-instructions.txt", &instructions);

    let tip = path.join("tip.txt");
    let _ = fs::copy("templates/small_change-tip.txt", &tip);

    // todo: need to create the path as with file.path()
    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();

    git::add_file(&path_str, "bla");
    git::commit(&path_str,"feat: add bla");

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