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
    let path = git::init_playground(&parent_path, "latest_message", lang);

    // Create the Git mess :-)
    let bla = path.join("bla");
    File::create(&bla).unwrap();
    fs::write(&bla, "thing1\nthing2").unwrap();

    git::add_file(&path, "bla");
    git::commit(&path,"add a new documennnt");

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
    fn tip_en_has_amend_shortcut() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        let tip = fs::read_to_string(format!("{}/tip.txt", dir)).unwrap();
        assert!(tip.contains("--amend -m"), "tip should include git commit --amend -m shortcut");
    }
}