# Changelog

## Unreleased

### Bug fixes

- Fixed optimized dependency configuration having invalid table keys in `Cargo.toml`.

## Version 0.1.0

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
