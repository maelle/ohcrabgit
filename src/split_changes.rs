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
    let path = git::init_playground(&parent_path, "split_changes", lang);

    // Create the Git mess :-)
    let script = path.join("script.txt");
    File::create(&script).unwrap();
    fs::write(&script, "a <- 1\nb <- 2").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: add script");

    // Three separate edits: top, middle, end — left uncommitted
    // Student uses git add --patch to stage them one by one
    fs::write(&script, "# a comment\na <- 1\n1/2\n1/3\nb <- 2\nlibrary(ggplot2)").unwrap();

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
    fn script_is_uncommitted() {
        use git2::{Repository, Status};
        use std::path::Path;

        let dir = exo("tempdir".to_string(), &Lang::En);
        let repo = Repository::open(Path::new(&dir)).unwrap();
        let statuses = repo.statuses(None).unwrap();
        let has_modified = statuses.iter().any(|s| {
            s.path().unwrap_or("") == "script.txt"
                && s.status().intersects(Status::WT_MODIFIED)
        });
        assert!(has_modified, "script.txt should have uncommitted changes");
    }
}
