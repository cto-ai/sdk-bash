# Contributing

Thanks for taking the time to contribute to SDK Bash!

This Bash SDK is a group of binaries enabling users to interact with the CTO.ai
API within their workflows. These binaries are written in Rust, so using `cargo`
for the development cycle should suffice; however, we would like to share some
details to help you with your contribution.

## Project structure

The `bash-sdk` project contains three Rust crates, which are not
combined into a Cargo workspace.

The base `bash-sdk` crate contains only the frontend programs `ctoai`,
`sdk`, and `ux`. These use the [clap library](https://lib.rs/clap) to
provide a command line interface with options and subcommands.

The commands themselves are defined in the `commands` crate. Each of
the toplevel commands is a Rust module, with any subcommands as
submodules. Each module exports its name as a constant string called
`CMD`, a function for returning the command definition for the
constructor function, called `init_cli_command`, and a function for
executing the command, called `run`. This structure is maintained at
all levels for consistency and encapsulation.

To make requests of the daemon and receive answers, the `sdk` crate is
used. This is actually a prerelease of a standalone Rust SDK which is
not ready for public consumption. The `commands` crate should limit
itself to defining the command line arguments and flags, and
formatting the output for outside consumption.

Most APIs in the `sdk` crate use a builder pattern, as is common in
Rust libraries. Required parameters are provided to the initial
constructor and optional parameters through method calls on the
builder struct.

## Build and release process

The Bash SDK binaries are automatically built and packaged as `tar.gz` by the
GitHub Workflows within this repository, on every pushed commit. The build logic is
distributed across the `scripts/build*` scripts, which make use of `docker buildx` to
cross compile the binaries. However, for local development, using `cargo build` should suffice.

When the master branch is tagged with a new version (which must match the
version in the root `Cargo.toml` configuration), a new release is built. The
resulting artifacts will be published through the **Releases** of this repo.
