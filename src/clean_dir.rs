use std::fs;
use std::fs::File;
use zut::git;
use zut::lang::Lang;
use zut::path;


pub fn exo(
    target: String,
    lang: &Lang,
) -> String {

    let parent_path = path::create_target(target);
    let path = git::init_playground(&parent_path, "clean_dir", lang);

    // Create the Git mess :-)
    let script = path.join("script.txt");
    File::create(&script).unwrap();
    fs::write(&script, "useful stuff").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: add script");

    // Untracked files: debugging/ folder with debug files
    let debugging = path.join("debugging");
    fs::create_dir(&debugging).unwrap();
    fs::write(debugging.join("debug1.txt"), "debug output 1").unwrap();
    fs::write(debugging.join("debug2.txt"), "debug output 2").unwrap();

    println!("{}", path.display());
    return path.to_str().unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        assert!(fs::exists(&dir).unwrap());
        assert!(fs::exists(format!("{}/.git", dir)).unwrap());
    }

    #[test]
    fn debugging_folder_exists_but_is_untracked() {
        use git2::{Repository, Status};
        use std::path::Path;

        let dir = exo("tempdir".to_string(), &Lang::En);
        assert!(fs::exists(format!("{}/debugging", dir)).unwrap(), "debugging/ folder should exist");

        let repo = Repository::open(Path::new(&dir)).unwrap();
        let statuses = repo.statuses(None).unwrap();
        let has_untracked = statuses.iter().any(|s| {
            s.path().unwrap_or("").starts_with("debugging")
                && s.status().intersects(Status::WT_NEW)
        });
        assert!(has_untracked, "debugging/ contents should be untracked in git");
    }
}
