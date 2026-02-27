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
    let path = git::init_playground(&parent_path, "worktree", lang);

    // Create the Git mess :-)
    // 10 flower commits, then create tags v1, v2, v3 at specific ancestry points
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

    // Tags: v1=HEAD~8 (tulip), v2=HEAD~5 (sunflower), v3=HEAD~2 (orchid)
    git::create_tag(&path, "v1", "HEAD~8");
    git::create_tag(&path, "v2", "HEAD~5");
    git::create_tag(&path, "v3", "HEAD~2");

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
    fn tags_exist() {
        use git2::Repository;
        use std::path::Path;

        let dir = exo("tempdir".to_string(), &Lang::En);
        let repo = Repository::open(Path::new(&dir)).unwrap();
        let tags = repo.tag_names(None).unwrap();
        let tag_list: Vec<_> = tags.iter().flatten().collect();
        assert!(tag_list.contains(&"v1"), "tag v1 should exist");
        assert!(tag_list.contains(&"v2"), "tag v2 should exist");
        assert!(tag_list.contains(&"v3"), "tag v3 should exist");
    }
}
