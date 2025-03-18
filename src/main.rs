use std::env;
use git2::Repository;
use git2::IndexAddOption;
use git2::Signature;
use git2::Time;
use std::fs;
use std::fs::File;
use time::macros::{datetime};

fn main() {
    let args: Vec<String> = env::args().collect();
    let exo = &args[1];
    if !exos().contains(&exo) {
        panic!("No exercise of this name!");
    }

    launch_exo(&exo);
}

fn exos()  -> Vec<String> {
    vec!["small_change".to_string()]
}

fn launch_exo(
    exo: &String
) {
    let to_match = exo.as_str();
    match to_match {
        "small_change" => small_change(),
        &_ => println!("{}", "error")
    }
}

fn small_change() {
    let path = "../exo-small-change";
    fs::create_dir(path).unwrap();
    Repository::init(path).unwrap();
    let repo = Repository::open(path).expect("failed to open");

    // todo: need to create the path as with file.path()
    let bla = "../exo-small-change/bla";
    File::create(bla).unwrap();
    fs::write(bla, "thing1\nthing2").unwrap();

    let mut index = repo.index().expect("cannot get the Index file");
    // todo add one only explicitly
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();

    let default_datetime = Time::new(
        datetime!(2023-12-15 15:25:00).assume_utc().unix_timestamp(), 
        0
    );
    let signature = Signature::new(
        "Jane Doe",
        "jane@example.com",
        &default_datetime
    ).expect("ouch");
    
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
       "feat: add bla",
        &tree,
        &[],
    ).unwrap();

    println!("{}", path)
}
