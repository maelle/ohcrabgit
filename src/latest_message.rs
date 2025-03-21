use std::fs;
use std::fs::File;
use std::env;
use ohcrabgit::git;
use rand::Rng;

pub fn exo() -> String {
    let tmp_dir = env::temp_dir();
    let random_string = rand::rng().random_range(1..1000001).to_string();
    let random_path = "ohcrabgit".to_string() + &random_string;
    let parent_path = tmp_dir.join(random_path);
    if !fs::exists(&parent_path).unwrap() {
        fs::create_dir(&parent_path).unwrap();
    }
    let path = parent_path.join("exo-latest_message");
    let path_str =  &path.to_str().unwrap();
    fs::create_dir(&path).expect(&path_str);
    git::repo_init(&path_str);

    let gitignore = path.join(".gitignore");
    let _ = fs::copy("templates/gitignore.txt", &gitignore);
    git::add_file(&path_str, ".gitignore");
    git::first_commit(&path_str, "git ignore");

    let instructions = path.join("instructions.txt");
    let _ = fs::copy("templates/latest_message-instructions.txt", &instructions);

    let tip = path.join("tip.txt");
    let _ = fs::copy("templates/latest_message-tip.txt", &tip);

    // todo: need to create the path as with file.path()
    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();

    git::add_file(&path_str, "bla");
    git::commit(&path_str,"add a new documennnt");

    println!("{}", path.display());
    return path.to_str().unwrap().to_string();
}

#[test]
fn it_works() {
    let dir = exo();
    println!("{}", dir);
    assert!(fs::exists(&dir).unwrap());

    let git_dir = dir + "/.git";
    assert!(fs::exists(&git_dir).unwrap());
}