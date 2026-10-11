# rrainn/cli

The Rust foundation for the rrainn command-line interface. It currently provides
help and version output; running it without arguments displays help.

## Build and run

Install a stable Rust toolchain supporting edition 2024 and the dependencies in
`Cargo.lock`, then build from the repository root:

```sh
cargo build --locked
cargo run --locked -- --help
cargo run --locked -- --version
```

For an optimized executable:

```sh
cargo build --locked --release
./target/release/rrainn --help
```

## Install locally

```sh
cargo install --locked --path .
rrainn --help
rrainn --version
```

Ensure Cargo's binary directory (normally `~/.cargo/bin`) is on your `PATH`.

`-h` is equivalent to `--help`, and `-V` is equivalent to `--version`.
Help and version requests exit successfully. Unknown options and unexpected
positional arguments produce diagnostics on stderr and exit with code 2.

Only the `rrainn` executable is installed. For a shorter interactive command,
optionally add this alias to your shell configuration (such as `~/.zshrc` or
`~/.bashrc`):

```sh
alias rn=rrainn
```

The alias takes precedence over other `rn` commands in that shell; use `rrainn`
in scripts for an unambiguous command name.

## Development checks

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

Rustfmt is configured to use tabs. Integration tests exercise the executable's
help, version, default invocation, and invalid argument behavior.
