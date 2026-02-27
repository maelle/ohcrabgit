pub mod time_machine;
pub mod small_change;
pub mod latest_message;
pub mod committed_to_main;
pub mod committed_to_wrong;
pub mod undo_commit;
pub mod undo_file;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{CommandFactory, FromArgMatches, Parser, ValueEnum};
use colored::Colorize;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Magenta.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Magenta.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Blue.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(version, about, long_about = None, styles = STYLES)]
#[clap(override_usage = "ohcrabgit <EXO>[TARGET]. In the exercise folder, open instructions.txt.")]
struct Cli {
    /// Name of the exercise
    #[arg(value_enum)]
    exo: Exo,
    /// Where to create the exercise directory. Default: temporary directory.
    #[arg(default_value = "tempdir")]
    target: String
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
    /// Oh shit, I accidentally committed to the wrong branch!
    CommittedToWrong,
    /// Oh shit, I need to undo a commit from like 5 commits ago!
    UndoCommit,
    /// Oh shit, I need to undo my changes to a file!
    UndoFile
}

fn main() {
    let after_help = format!(
        "Examples:\n\n  {sc}  creates the small-change exercise folder in a temporary folder.\
         \n  {lm}  creates the latest-message exercise folder in the parent of the current folder.",
        sc = "ohcrabgit small-change".blue().bold(),
        lm = "ohcrabgit latest-message ..".blue().bold(),
    );
    let cmd = Cli::command().after_help(after_help);
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    let target = cli.target;

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
        Exo::CommittedToWrong => {
            committed_to_wrong::exo(target);
        }
        Exo::UndoCommit => {
            undo_commit::exo(target);
        }
        Exo::UndoFile => {
            undo_file::exo(target);
        }
    }
}

