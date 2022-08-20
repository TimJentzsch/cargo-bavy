# cargo-bavy [![Crates.io](https://img.shields.io/crates/v/cargo-bavy.svg)](https://crates.io/crates/cargo-bavy) [![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/TimJentzsch/cargo-bavy/#license)

A third-party command-line utility for the [Bevy game engine](https://bevyengine.org/).

To not steal the `cargo bevy` command from the official Bevy maintainers, I chose `cargo bavy` instead.

## Installation

```cli
cargo install cargo-bavy
```

## Usage

### Creating a new Bevy app

Run the following command:

```cli
cargo bavy new <FOLDER_NAME>
```

You can then select all the features that you want.
Afterwards, a template project with the given name is created for you.

### Running your game

Run the following command:

```cli
cargo bavy run
```

This works similar to `cargo run`, but automatically passes the `--features bevy/dynamic` flag to get faster compile times.

It also provides a `--wasm`/`-w` flag, which allows you to quickly **test your game for the web**.
It will automatically compile your game for `wasm32-unknown-unknown`, bundle it with `wasm-bindgen-cli` and then serve it on a local webserver to be playable in your browser.
All necessary tools will be installed for you if needed.

### Building your game

Run the following command:

```cli
cargo bavy build
```

This works similar to `cargo build`, but automatically passes the `--features bevy/dynamic` flag to get faster compile times.

It also provides a `--wasm`/`-w` flag, which allows you to create a bundle for the web.
It will automatically compile your game for `wasm32-unknown-unknown` and bundle it with `wasm-bindgen-cli`.
All necessary tools will be installed for you if needed.

### Check your code

Run the following command:

```cli
cargo bavy check
```

This works similar to `cargo check`, but automatically passes the `--features bevy/dynamic` flag to get faster compile times.

It also provides a `--wasm`/`-w` flag, which allows you to check for the WASM target.

### Notes for usage with Rust Analyzer

If you are using `cargo bavy` with Rust Analyzer, you should configure it to use the `--features bevy/dynamic` flag in your project.
Otherwise, its checks might interfere with your `cargo bavy run` usage, leading to many non-incremental re-compiles.

Create a `.vscode/settings.json` file in your project with the following content:

```json
{
  "rust-analyzer.cargo.features": ["bevy/dynamic"]
}
```

This way, Rust Analyzer uses the same features as you.

## License

The source code of this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
