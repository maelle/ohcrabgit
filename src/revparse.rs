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
    let path = git::init_playground(&parent_path, "revparse", lang);

    // Create the Git mess :-)
    // 10 commits, each overwriting flowers.txt with one flower name
    // Student uses git rev-parse to understand HEAD~5 and HEAD^^
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
    fn head_minus_5_is_sunflower() {
        use git2::Repository;
        use std::path::Path;

        let dir = exo("tempdir".to_string(), &Lang::En);
        let repo = Repository::open(Path::new(&dir)).unwrap();
        let commit = repo.revparse_single("HEAD~5").unwrap().peel_to_commit().unwrap();
        assert_eq!(commit.message().unwrap().trim(), "feat: graceful");
    }
}
