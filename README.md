
# ohcrabgit

<!-- badges: start -->
<!-- badges: end -->

The goal of ohcrabgit is for me to learn a bit of Rust by re-creating my [saperlipopette](https://docs.ropensci.org/saperlipopette/) R package as a CLI called "ohcrabgit".

## Current exercises

Currently only `ohcrabgit small-change` and `ohcrabgit latest-message` will work and create a folder `exo-small_change` in a temporary directory, whose name starts with `ohcrabgit`.
It will print the folder name so that you might open it.
In this folder, open the `instructions.txt` file to know what the exercise is about.

## Development notes

Use `--` as separator between arguments for cargo and arguments for ohcrabgit.

```
cargo run -- -e small-change
```

This was helpful to implement the interface for possible values: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#enumerated-values
