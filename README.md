
# ohcrabgit

<!-- badges: start -->
<!-- badges: end -->

The goal of ohcrabgit is for me to learn a bit of Rust by re-creating my [saperlipopette](https://docs.ropensci.org/saperlipopette/) R package.

## Current exercises

Currently only `ohcrabgit -e small_change` will work and create a folder `exo-small_change` in a temporary directory, whose name starts with `ohcrabgit`.
It will print the folder name so that you might open it.
In this folder, open the `instructions.txt` file to know what the exercise is about.

The binary lives in `target/release/ohcrabgit` but I need to remember to update it.

## Development notes

Use `--` as separator between arguments for cargo and arguments for ohcrabgit.

```
cargo run -- -e small_change
```

## Current issues/ideas

- display a list of possible exercises.
- how to document the thing.
- add tests.
- add CI for building the target.
- add all saperlipopette exercises.
- ...

