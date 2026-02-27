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
    let path = git::init_playground(&parent_path, "bisect", lang);

    // Create the Git mess :-)
    let script = path.join("script.sh");
    File::create(&script).unwrap();
    fs::write(&script, "#!/bin/sh\nset -e\na=1\nb=2\n").unwrap();
    git::add_file(&path, "script.sh");
    git::commit(&path, "feat: add script");

    // 100 commits, appending one line each time
    // At i=13, the "bug" line "aaaaaaaaah" is introduced — causes sh to exit non-zero
    for i in 1..=100_u32 {
        let current = fs::read_to_string(&script).unwrap();
        let new_line = if i == 13 {
            "aaaaaaaaah".to_string()
        } else {
            format!("x=$((1+{}))", i)
        };
        fs::write(&script, format!("{}{}\n", current, new_line)).unwrap();
        git::add_file(&path, "script.sh");
        git::commit(&path, "feat: edit script");
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
    fn script_sh_is_executable_at_head() {
        use std::process::Command;

        let dir = exo("tempdir".to_string(), &Lang::En);
        // HEAD version has "aaaaaaaaah" — sh should exit non-zero
        let status = Command::new("sh")
            .arg(format!("{}/script.sh", dir))
            .status()
            .unwrap();
        assert!(!status.success(), "script.sh should fail at HEAD (bug present)");
    }
}
