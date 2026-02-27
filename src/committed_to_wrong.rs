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
    let path = git::init_playground(&parent_path, "committed_to_wrong", lang);

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
    fn tip_en_has_git_switch() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        let tip = fs::read_to_string(format!("{}/tip.txt", dir)).unwrap();
        assert!(tip.contains("git switch feat-bla"), "tip should mention git switch feat-bla");
    }
}