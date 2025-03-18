use std::fs;
use std::fs::File;
use std::path::Path;
use ohcrabgit::git;

pub fn exo() {
    let path_str =  "../exo-small-change";
    let path = Path::new(&path_str);
    fs::create_dir(path).unwrap();
    git::repo_init(&path_str);

    let gitignore = path.join(".gitignore");
    let _ = fs::copy("templates/gitignore.txt", &gitignore);
    git::add_file(&path_str, ".gitignore");
    let message = "git ignore";
    git::first_commit(&path_str, &message);

    let instructions = path.join("instructions.txt");
    let _ = fs::copy("templates/small_change-instructions.txt", &instructions);

    let tip = path.join("tip.txt");
    let _ = fs::copy("templates/small_change-tip.txt", &tip);

    // todo: need to create the path as with file.path()
    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();

    git::add_file(&path_str, "bla");
    let message = "feat: add bla";
    git::commit_all(&path_str, &message);

    println!("{}", path.display())
}