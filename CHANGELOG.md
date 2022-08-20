# Changelog

## Unreleased

### Enhancements

- Added `cargo bavy run` command:
  - Works similar to `cargo` counterpart.
  - Automatically adds `--features bevy/dynamic` in debug mode for faster compile times.
  - Has additional `--wasm`/`-w` flag to run game in the browser.
  - All necessary tools can be installed for you.
- A `.gitignore` file will now be added to the `wasm/` folder.

### Bug fixes

- Fixed optimized dependency configuration having invalid table keys in `Cargo.toml`.
- Fixed fast linker configuration including a shared generics flag on stable toolchain, which caused a compile error.

## Release v0.1.0 - 2022-08-19

The initial release!

### Installation

```cli
cargo install cargo-bavy
```

### Usage

Create a new Bevy app:

```cli
cargo bavy new <PROJECT_NAME>
```

It will ask you which features you want (asset hot reloading, fast linker, GitHub Action workflows, ...) and then creates a project template for you.
