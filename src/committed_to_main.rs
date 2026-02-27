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
    let path = git::init_playground(&parent_path, "committed_to_main", lang);

    // Create the Git mess :-)
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
        assert!(tip.contains("git switch main"), "tip should mention git switch main");
    }

    #[test]
    fn instructions_es_content() {
        let dir = exo("tempdir".to_string(), &Lang::Es);
        let instructions = fs::read_to_string(format!("{}/instructions.txt", dir)).unwrap();
        assert!(instructions.contains("Mierda"), "ES instructions should be in Spanish");
    }
}
