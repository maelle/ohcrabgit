
# zut

<!-- badges: start -->
<!-- badges: end -->

The goal of zut is for me to learn a bit of Rust by re-creating my [saperlipopette](https://docs.ropensci.org/saperlipopette/) R package as a CLI called "zut".

## Installation

Requires [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) (Cargo comes with Rust).

```
cargo install --git https://github.com/maelle/zut
```

## Current exercises

18 exercises in 3 categories. Run `zut --help` for the full list.

### Oh shit, Git!

- `time-machine` — Oh shit, I did something terribly wrong, please tell me git has a magic time machine!?!
- `small-change` — Oh shit, I committed and immediately realized I need to make one small change!
- `latest-message` — Oh shit, I need to change the message on my last commit!
- `committed-to-main` — Oh shit, I accidentally committed something to main that should have been on a brand new branch!
- `committed-to-wrong` — Oh shit, I accidentally committed to the wrong branch!
- `undo-commit` — Oh shit, I need to undo a commit from like 5 commits ago!
- `undo-file` — Oh shit, I need to undo my changes to a file!

### Clean history

- `split-changes` — Hey I'd like to split these changes to the same file into several commits!
- `clean-dir` — Hey, how do I remove all my debugging left-over stuff at once?
- `conflict` — Hey I'd like to see what merge conflicts look like!
- `rebase-i` — Hey I'd like to make my commits in a branch look informative and smart!
- `reset` — Hey I'd like to restart from scratch and reorganize my commits!

### Use history

- `bisect` — Hey I'd like to find which commit introduced a bug!
- `log-deleted-file` — I want to find which commit deleted a file!
- `log-deleted-line` — I want to find which commit deleted a line!
- `revparse` — I want to understand ancestry references like HEAD~5 and HEAD^^!
- `blame` — I want to find who added a specific line and when!
- `worktree` — I need to see what the project looked like at a certain version!

## Why the name

Many exercises follow ["Oh shit, Git!"](https://ohshitgit.com/) by Katie Sylor-Miller.
"zut" is a nice French curseword.

## Acknowledgements

The exercises and their content are ported from the [saperlipopette](https://docs.ropensci.org/saperlipopette/) R package, co-authored by [Yanina Bellini Saibene](https://yabellini.netlify.app/) and [Jim Gardner](https://github.com/jimgar).

## Dev notes

Use `--` as separator between arguments for cargo and arguments for zut.

```
cargo run -- small-change
```

This was helpful to implement the interface for possible values: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#enumerated-values
