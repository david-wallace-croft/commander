# CroftSoft Commander

## Purpose
- Command-line Rust example
  - My first open source Rust program
- Demonstrates using the Command Line Argument Parser (clap) for Rust
- Demonstrates using a custom alternative library instead of clap
  - I wanted to learn how to parse arguments as part of learning Rust
  - Shows how to configure a Rust project to use an alternative main file

## Usage

- cargo run
  - Runs the default main that uses the clap library
- cargo run --bin commander-args-lib
  - Runs the other main that uses the alternative parser library
- cargo run -- --help
- cargo run -- --interactive false
- cargo run -- --name World
