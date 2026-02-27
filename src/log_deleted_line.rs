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
    let path = git::init_playground(&parent_path, "log_deleted_line", lang);

    // Create the Git mess :-)
    // 10 commits, each overwriting flowers.txt with one flower name
    // "iris" was in commit 9 but replaced by "poppy" in commit 10
    // Student finds which commit deleted the "iris" line using git log -S
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
    fn flowers_txt_ends_with_poppy() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        let content = fs::read_to_string(format!("{}/flowers.txt", dir)).unwrap();
        assert_eq!(content.trim(), "poppy");
    }
}
