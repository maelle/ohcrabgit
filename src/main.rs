pub mod small_change;
pub mod latest_message;
pub mod time_machine;
pub mod committed_to_main;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[clap(override_usage = "ohcrabgit <EXO>[TARGET]. In the exercise folder, open instructions.txt.")]
#[clap(after_help = "Examples:

`ohcrabgit small-change` creates the small-change exercise folder in a temporary folder.
`ohcrabgit latest-message ..` creates the latest-message exercise folder in the parent of the current folder.")]
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
    /// Oh shit, I did something terribly wrong, please tell me git has a magic time machine!?!
    TimeMachine,
    /// Oh shit, I committed and immediately realized I need to make one small change!
    SmallChange,
    /// Oh shit, I need to change the message on my last commit!
    LatestMessage,
    /// Oh shit, I accidentally committed something to master that should have been on a brand new branch!
    CommittedToMain,
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
        Exo::TimeMachine => {
            time_machine::exo(target);
        }
        Exo::CommittedToMain => {
            committed_to_main::exo(target);
        }
    }
}

