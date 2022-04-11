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

### Development Commands

- cargo clippy
- cargo doc --examples --no-deps
  - Makes target/doc/commander/index.html
- cargo fmt
- cargo test
- cargo test --all-targets
- cargo update
- rustup update

## Project Structure

- A Rust workspace
- With a lib crate "croftsoft-commander"
- And another lib crate "croftsoft-commander-examples"
  - With two examples
    - One showing how to use the lib "croftsoft-commander"
    - The other showing how to use the lib "clap"

## TODO

- Unit tests for when the default option value is true
