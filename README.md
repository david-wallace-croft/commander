# CroftSoft Commander

## Purpose

- A command-line arguments parser library
  - An alternative to using the [Command Line Argument Parser](https://github.com/clap-rs/clap) (clap) for Rust

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

- cargo run --example example-commander -- --help
  - Prints the help information for the examples that uses the library
- cargo run --example example-commander
  - Runs the example that uses the library
- cargo run --example example-commander -- -i false
  - Disable interactive mode
- cargo run --example example-commander -- -n World
  - Provide the default name
- cargo run --example example-commander -- -i false -n World
  - Disable interactive mode and use the specified name
- cargo run --example example-commander -- --interactive false
  - Disable interactive mode
- cargo run --example example-commander -- --name World
  - Provide the default name

### clap Example

- Usage for the clap Example is similar to the Parser Library Example
  - Except that the commands start with "cargo run --example example-clap"

## Project Structure

- A Rust workspace with two crates
- The primary library crate "croftsoft-commander"
  - Functions for parsing option values from command-line arguments
  - Functions for displaying application help information including options
- The secondary library crate "croftsoft-commander-examples"
  - With two example applications
    - One showing how to use the library crate "croftsoft-commander"
    - The other showing how to run the same example application using "clap"
  - And a library containing the application code common to the two examples
    - A "Hello, World" application that prompts the user for a name
    - Configured by parsing option values from command-line arguments
    - Using either the "croftsoft-commander" library or the "clap" library

### Development Commands

- cargo clippy
- cargo doc --examples --no-deps
  - Makes target/doc/commander/index.html
- cargo fmt
- cargo test
- cargo test --all-targets
- cargo update
- rustup update

## TODO

- Support the equals sign for the value of an option
- Unit tests for when the default option value is true
