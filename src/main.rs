pub mod small_change;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the exercise
    #[arg(short, long)]
    exo: String
}


fn main() {
    let args = Args::parse();

    let exo = &args.exo;
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
