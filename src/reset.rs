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
    let path = git::init_playground(&parent_path, "reset", lang);

    // Create the Git mess :-)
    // Create feature branch with 7 messy commits (same setup as rebase_i)
    git::create_branch(&path, "feature");

    let ci = path.join("ci.yml");
    let bla = path.join("bla.txt");

    File::create(&ci).unwrap();
    fs::write(&ci, "do: yes").unwrap();
    git::add_file(&path, "ci.yml");
    git::commit(&path, "add ci configuration");

    File::create(&bla).unwrap();
    fs::write(&bla, "1/0").unwrap();
    git::add_file(&path, "bla.txt");
    git::commit(&path, "add script");

    fs::write(&ci, "do: true").unwrap();
    git::add_file(&path, "ci.yml");
    git::commit(&path, "try to fix ci");

    fs::write(&ci, "do: 1").unwrap();
    git::add_file(&path, "ci.yml");
    git::commit(&path, "try to fix ci");

    fs::write(&bla, "1/Inf").unwrap();
    git::add_file(&path, "bla.txt");
    git::commit(&path, "try to fix script");

    fs::write(&ci, "do: 1\nsave: 1").unwrap();
    git::add_file(&path, "ci.yml");
    git::commit(&path, "add a ci thing");

    fs::write(&bla, "1/2").unwrap();
    git::add_file(&path, "bla.txt");
    git::commit(&path, "fix script");

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
    fn bla_txt_final_content() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        let content = fs::read_to_string(format!("{}/bla.txt", dir)).unwrap();
        assert_eq!(content.trim(), "1/2");
    }
}
