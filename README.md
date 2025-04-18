
# ohcrabgit

<!-- badges: start -->
<!-- badges: end -->

The goal of ohcrabgit is for me to learn a bit of Rust by re-creating my [saperlipopette](https://docs.ropensci.org/saperlipopette/) R package as a CLI called "ohcrabgit".

## Installation

This needs to be improved. :sweat_smile:

- The binary for ohcrabgit is built at each commit, on GitHub Actions, and available as an artifact.
- You can build ohcrabgit from source using `cargo build --release`. This means having to [install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) (Cargo comes with Rust).

## Current exercises

To see help

```
ohcrabgit --help
```

```
Usage: ohcrabgit <EXO>[TARGET]. In the exercise folder, open instructions.txt.

Arguments:
  <EXO>
          Name of the exercise

          Possible values:
          - small-change:   Oh shit, I committed and immediately realized I need to make one small change!
          - latest-message: Oh shit, I need to change the message on my last commit!
          - time-machine:   Oh shit, I did something terribly wrong, please tell me git has a magic time machine!?!
          - committed-to-main: Oh shit, I accidentally committed something to master that should have been on a brand new branch!
          - committed-to-wrong: Oh shit, I accidentally committed to the wrong branch!
          - undo-commit:        Oh shit, I need to undo a commit from like 5 commits ago!
          - undo-file:          Oh shit, I need to undo my changes to a file!

  [TARGET]
          Where to create the exercise directory. Default: temporary directory
          
          [default: tempdir]

Examples:

`ohcrabgit small-change` creates the small-change exercise folder in a temporary folder.
`ohcrabgit latest-message ..` creates the latest-message exercise folder in the parent of the current folder.
```

## Why the name

Many exercises follow ["Oh shit, Git!"](https://ohshitgit.com/) by Katie Sylor-Miller.
"Oh crap, Git!" would mean the same.
"Oh crab, Git!" refers to Rust. :crab:
I am open to changing the name to something easier to type later on.

## Dev notes

Use `--` as separator between arguments for cargo and arguments for ohcrabgit.

```
cargo run -- small-change
```

This was helpful to implement the interface for possible values: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#enumerated-values
