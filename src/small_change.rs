

use std::fs;
use std::fs::File;
use ohcrabgit::git;

pub fn exo() {
    let path = "../exo-small-change";
    fs::create_dir(path).unwrap();
    git::repo_init(path);

    let gitignore = format!("{}/.gitignore", path);
    let _ = fs::copy("templates/gitignore.txt", &gitignore);
    git::add_file(&path, ".gitignore");
    let message = "git ignore";
    git::first_commit(&path, &message);

    let instructions = format!("{}/instructions.txt", path);
    let _ = fs::copy("templates/small_change-instructions.txt", &instructions);

    let tip = format!("{}/tip.txt", path);
    let _ = fs::copy("templates/small_change-tip.txt", &tip);

    // todo: need to create the path as with file.path()
    let bla = format!("{}/bla", path);
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();

    git::add_file(&path, "bla");
    let message = "feat: add bla";
    git::commit_all(&path, &message);

    println!("{}", path)
}