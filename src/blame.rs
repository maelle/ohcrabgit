use std::fs;
use std::fs::File;
use time::macros::datetime;
use zut::git;
use zut::lang::Lang;
use zut::path;


pub fn exo(
    target: String,
    lang: &Lang,
) -> String {

    let parent_path = path::create_target(target);
    let path = git::init_playground(&parent_path, "blame", lang);

    // Create the Git mess :-)
    // 5 commits to script.txt alternating Dr Jekyll and Mr Hyde
    // "x <- x + 1" was added in commit 2 by Mr Hyde
    let script = path.join("script.txt");
    File::create(&script).unwrap();

    // Commit 1: Dr Jekyll
    fs::write(&script, "a <- 1\nb <- 2\n").unwrap();
    git::add_file(&path, "script.txt");
    git::commit_with_author(
        &path,
        "feat: add script",
        "Dr Jekyll",
        "jekyll@example.com",
        datetime!(2024-01-05 9:00:00).assume_utc().unix_timestamp(),
    );

    // Commit 2: Mr Hyde — adds the suspicious line
    fs::write(&script, "a <- 1\nb <- 2\nx <- x + 1\n").unwrap();
    git::add_file(&path, "script.txt");
    git::commit_with_author(
        &path,
        "feat: edit script",
        "Mr Hyde",
        "hyde@example.com",
        datetime!(2024-01-06 23:00:00).assume_utc().unix_timestamp(),
    );

    // Commit 3: Dr Jekyll
    fs::write(&script, "a <- 1\nb <- 2\nx <- x + 1\nc <- 3").unwrap();
    git::add_file(&path, "script.txt");
    git::commit_with_author(
        &path,
        "feat: improve script",
        "Dr Jekyll",
        "jekyll@example.com",
        datetime!(2024-01-08 10:00:00).assume_utc().unix_timestamp(),
    );

    // Commit 4: Mr Hyde
    fs::write(&script, "a <- 1\nb <- 20\nx <- x + 1\nc <- 3").unwrap();
    git::add_file(&path, "script.txt");
    git::commit_with_author(
        &path,
        "feat: amend script",
        "Mr Hyde",
        "hyde@example.com",
        datetime!(2024-01-09 22:00:00).assume_utc().unix_timestamp(),
    );

    // Commit 5: Dr Jekyll
    fs::write(&script, "a <- 10\nb <- 20\nx <- x + 1\nc <- 3").unwrap();
    git::add_file(&path, "script.txt");
    git::commit_with_author(
        &path,
        "feat: edit script",
        "Dr Jekyll",
        "jekyll@example.com",
        datetime!(2024-01-10 11:00:00).assume_utc().unix_timestamp(),
    );

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
    fn script_contains_suspicious_line() {
        let dir = exo("tempdir".to_string(), &Lang::En);
        let content = fs::read_to_string(format!("{}/script.txt", dir)).unwrap();
        assert!(content.contains("x <- x + 1"), "script.txt should contain the suspicious line");
    }
}
