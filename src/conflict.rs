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
    let path = git::init_playground(&parent_path, "conflict", lang);

    // Create the Git mess :-)
    let script = path.join("script.txt");
    File::create(&script).unwrap();
    fs::write(&script, "a <- 1\nb <- 2").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: add script");

    // Remember default branch name before switching
    let default_branch = git::current_branch(&path);

    // Create feature branch, make a diverging commit
    git::create_branch(&path, "feature");
    fs::write(&script, "a <- 10\nb <- 2\nc <- 3").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: improve script");

    // Go back to default branch, make a conflicting commit
    git::checkout_branch(&path, &default_branch);
    fs::write(&script, "a <- 11\nb <- 2").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: amend script");

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
    fn feature_branch_exists() {
        use git2::Repository;
        use std::path::Path;

        let dir = exo("tempdir".to_string(), &Lang::En);
        let repo = Repository::open(Path::new(&dir)).unwrap();
        assert!(
            repo.find_branch("feature", git2::BranchType::Local).is_ok(),
            "feature branch should exist"
        );
    }
}
