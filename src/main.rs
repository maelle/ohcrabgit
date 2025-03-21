pub mod small_change;
pub mod latest_message;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the exercise
    #[arg(value_enum)]
    exo: Exo
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

    match cli.exo {
        Exo::SmallChange => {
            small_change::exo();
        }
        Exo::LatestMessage => {
            latest_message::exo();
        }
    }
}

