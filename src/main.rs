use std::env;
pub mod small_change;

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
        "small_change" => small_change::exo(),
        &_ => println!("{}", "error")
    }
}
