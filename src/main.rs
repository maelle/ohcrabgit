pub mod small_change;
pub mod latest_message;
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
    vec!["small_change".to_string(), "latest_message".to_string()]
}

fn launch_exo(
    exo: &String
) -> String {
    let to_match = exo.as_str();
    match to_match {
        "small_change" => small_change::exo(),
        "latest_message" => latest_message::exo(),
        &_ => "Unreachable".to_string()
    }
}
