pub mod small_change;
pub mod latest_message;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[clap(override_usage = "ohcrabgit <EXO>[TARGET]. In the exercise folder, open instructions.txt.")]
struct Cli {
    /// Name of the exercise
    #[arg(value_enum)]
    exo: Exo,
    #[arg(default_value = "tempdir")]
    /// Where to create the exercise directory. Default: temporary directory.
    target: Option<String>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Exo {
    /// Oh shit, I committed and immediately realized I need to make one small change!
    SmallChange,
    /// Oh shit, I need to change the message on my last commit!
    LatestMessage,
}

fn main() {
    let cli = Cli::parse();

    let target = cli.target.unwrap();

    match cli.exo {
        Exo::SmallChange => {
            small_change::exo(target);
        }
        Exo::LatestMessage => {
            latest_message::exo(target);
        }
    }
}

