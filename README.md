# CroftSoft Commander

## Purpose

- A command-line arguments parser library
  - An alternative to using the Command Line Argument Parser (clap) for Rust

## History

- My first open source Rust program
- Started off as an example of a Rust-based command-line program
  - To be copied as a template to start new projects
  - The project originally used clap to parse the command-line arguments
- Morphed into a parser library instead
  - I decided I should learn how to parse command-line arguments myself
  - As part of my learning the basics of the Rust programming language

## Usage

### Parser Library Example

- cargo run --example commander -- --help
  - Prints the help information for the examples that uses the library
- cargo run --example commander
  - Runs the example that uses the library
- cargo run --example commander -- -i false
  - Disable interactive mode
- cargo run --example commander -- -n World
  - Provide the default name
- cargo run --example commander -- -i false -n World
  - Disable interactive mode and use the specified name
- cargo run --example commander -- --interactive false
  - Disable interactive mode
- cargo run --example commander -- --name World
  - Provide the default name

### clap Example

- Usage for the clap Example is similar to the Parser Library Example
  - Except that the commands start with "cargo run --example clap"

### Development Commands

- cargo clippy
- cargo doc --no-deps
  - Makes target/doc/commander/index.html
- cargo fmt
- cargo test
- rustup update

## TODO

- Unit tests for when the default option value is true
- Move the clap example code to another project
  - Remove this clap dependency
