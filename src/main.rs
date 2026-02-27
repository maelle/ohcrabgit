pub mod time_machine;
pub mod small_change;
pub mod latest_message;
pub mod committed_to_main;
pub mod committed_to_wrong;
pub mod undo_commit;
pub mod undo_file;
pub mod clean_dir;
pub mod split_changes;
pub mod conflict;
pub mod rebase_i;
pub mod reset;
pub mod bisect;
pub mod log_deleted_file;
pub mod log_deleted_line;
pub mod revparse;
pub mod blame;
pub mod worktree;
use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{CommandFactory, FromArgMatches, Parser, ValueEnum};
use colored::Colorize;
use zut::lang::Lang;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Magenta.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Magenta.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Blue.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(version, about, long_about = None, styles = STYLES)]
#[clap(override_usage = "zut <EXO>[TARGET]. In the exercise folder, open instructions.txt.")]
struct Cli {
    /// Name of the exercise
    #[arg(value_enum)]
    exo: Exo,
    /// Where to create the exercise directory. Default: temporary directory.
    #[arg(default_value = "tempdir")]
    target: String,
    /// Language (default: auto-detected from system locale)
    #[arg(long, value_enum)]
    lang: Option<Lang>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Exo {
    // Oh shit, Git!
    /// Oh shit, I did something terribly wrong, please tell me git has a magic time machine!?!
    TimeMachine,
    /// Oh shit, I committed and immediately realized I need to make one small change!
    SmallChange,
    /// Oh shit, I need to change the message on my last commit!
    LatestMessage,
    /// Oh shit, I accidentally committed something to main that should have been on a brand new branch!
    CommittedToMain,
    /// Oh shit, I accidentally committed to the wrong branch!
    CommittedToWrong,
    /// Oh shit, I need to undo a commit from like 5 commits ago!
    UndoCommit,
    /// Oh shit, I need to undo my changes to a file!
    UndoFile,
    // Clean history
    /// Hey I'd like to split these changes to the same file into several commits!
    SplitChanges,
    /// Hey, how do I remove all my debugging left-over stuff at once?
    CleanDir,
    /// Hey I'd like to see what merge conflicts look like!
    Conflict,
    /// Hey I'd like to make my commits in a branch look informative and smart! (interactive rebase)
    RebaseI,
    /// Hey I'd like to restart from scratch and reorganize my commits! (git reset --mixed)
    Reset,
    // Use history
    /// Hey I'd like to find which commit introduced a bug!
    Bisect,
    /// I want to find which commit deleted a file!
    LogDeletedFile,
    /// I want to find which commit deleted a line!
    LogDeletedLine,
    /// I want to understand ancestry references like HEAD~5 and HEAD^^!
    Revparse,
    /// I want to find who added a specific line and when!
    Blame,
    /// I need to see what the project looked like at a certain version!
    Worktree,
}

fn main() {
    let after_help = format!(
        "Examples:\n\n  {sc}  creates the small-change exercise folder in a temporary folder.\
         \n  {lm}  creates the latest-message exercise folder in the parent of the current folder.\
         \n\nCategories:\n\n  {oh}:\n    time-machine, small-change, latest-message, committed-to-main,\n    committed-to-wrong, undo-commit, undo-file\
         \n\n  {ch}:\n    split-changes, clean-dir, conflict, rebase-i, reset\
         \n\n  {uh}:\n    bisect, log-deleted-file, log-deleted-line, revparse, blame, worktree",
        sc = "zut small-change".blue().bold(),
        lm = "zut latest-message ..".blue().bold(),
        oh = "Oh shit, Git!".bold(),
        ch = "Clean history".bold(),
        uh = "Use history".bold(),
    );
    let cmd = Cli::command().after_help(after_help);
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|e| e.exit());

    let target = cli.target;
    let lang = Lang::resolve(cli.lang);

    match cli.exo {
        Exo::SmallChange => {
            small_change::exo(target, &lang);
        }
        Exo::LatestMessage => {
            latest_message::exo(target, &lang);
        }
        Exo::TimeMachine => {
            time_machine::exo(target, &lang);
        }
        Exo::CommittedToMain => {
            committed_to_main::exo(target, &lang);
        }
        Exo::CommittedToWrong => {
            committed_to_wrong::exo(target, &lang);
        }
        Exo::UndoCommit => {
            undo_commit::exo(target, &lang);
        }
        Exo::UndoFile => {
            undo_file::exo(target, &lang);
        }
        Exo::CleanDir => {
            clean_dir::exo(target, &lang);
        }
        Exo::SplitChanges => {
            split_changes::exo(target, &lang);
        }
        Exo::Conflict => {
            conflict::exo(target, &lang);
        }
        Exo::RebaseI => {
            rebase_i::exo(target, &lang);
        }
        Exo::Reset => {
            reset::exo(target, &lang);
        }
        Exo::Bisect => {
            bisect::exo(target, &lang);
        }
        Exo::LogDeletedFile => {
            log_deleted_file::exo(target, &lang);
        }
        Exo::LogDeletedLine => {
            log_deleted_line::exo(target, &lang);
        }
        Exo::Revparse => {
            revparse::exo(target, &lang);
        }
        Exo::Blame => {
            blame::exo(target, &lang);
        }
        Exo::Worktree => {
            worktree::exo(target, &lang);
        }
    }
}
