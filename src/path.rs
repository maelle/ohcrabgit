
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::fs;
use rand::Rng;

pub fn create_target(
   target: String
) -> PathBuf {
    let parent_path = if target == "tempdir" {
        let tmp_dir = env::temp_dir();
        let random_string = rand::rng().random_range(1..1000001).to_string();
        let random_path = "ohcrabgit".to_string() + &random_string;
        tmp_dir.join(random_path)
    } else {
        Path::new(&target).to_path_buf()
    };
    if !fs::exists(&parent_path).unwrap() {
        fs::create_dir(&parent_path).unwrap();
    }

    return parent_path;
}