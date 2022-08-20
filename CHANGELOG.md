# Changelog

## Unreleased

### Enhancements

- Added `cargo bavy check`, `cargo bavy build` and `cargo bavy run` commands:
  - They work similar to `cargo` their counterpart.
  - They automatically add `--features bevy/dynamic` in debug mode for faster compile times.
  - They have an additional `--wasm`/`-w` flag to target the browser.
  - All necessary tools will be installed for you if needed.
- Changed `cargo bavy new` to add `.gitignore` file to the `wasm/` folder with the WASM option.
- Changed `cargo bavy new` to automatically install needed tools when WASM option is selected.

### Usability

- Changed `cargo bavy new` to colorize output.
- Changed `cargo bavy new` to suggest running `cargo bavy run` instead of `cargo run`.

### Bug fixes

- Fixed optimized dependency configuration having invalid table keys in `Cargo.toml`.
- Fixed fast linker configuration including a shared generics flag on stable toolchain, which caused a compile error.
- Fixed Clippy CI workflow not including GitHub token, so it wouldn't post comments with the warnings on your PRs.

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
