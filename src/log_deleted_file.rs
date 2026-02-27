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
    let path = git::init_playground(&parent_path, "log_deleted_file", lang);

    // Create the Git mess :-)
    let script = path.join("script.txt");
    File::create(&script).unwrap();
    fs::write(&script, "a <- 1\nb <- 2").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: add script");

    fs::write(&script, "a <- 10\nb <- 2\nc <- 3").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: improve script");

    fs::write(&script, "a <- 10\nb <- 20\nc <- 3").unwrap();
    git::add_file(&path, "script.txt");
    git::commit(&path, "feat: edit script");

    // Delete the file and stage the deletion
    git::remove_file(&path, "script.txt");
    git::commit(&path, "refactor: remove script");

    // 10 flower commits to bury the deletion in history
    let flowers = [
        "daisy", "tulip", "rose", "lavender", "sunflower",
        "jasmine", "orchid", "peony", "iris", "poppy",
    ];
    let messages = [
        "feat: wonderful", "feat: delightful", "feat: brilliant",
        "feat: cheerful", "feat: graceful", "feat: joyful",
        "feat: peaceful", "feat: radiant", "feat: splendid", "feat: vibrant",
    ];
    let flowers_file = path.join("flowers.txt");
    for (flower, message) in flowers.iter().zip(messages.iter()) {
        File::create(&flowers_file).unwrap();
        fs::write(&flowers_file, flower).unwrap();
        git::add_file(&path, "flowers.txt");
        git::commit(&path, message);
    }

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
    fn script_txt_is_deleted() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        assert!(
            !fs::exists(format!("{}/script.txt", dir)).unwrap(),
            "script.txt should be deleted"
        );
    }

    #[test]
    fn flowers_txt_exists() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        let content = fs::read_to_string(format!("{}/flowers.txt", dir)).unwrap();
        assert_eq!(content.trim(), "poppy");
    }
}
