# Contributing

Thanks for taking the time to contribute to SDK Bash! 🎉🎉🎉

The Bash SDK is a group of binaries enabling users to interact with the CTO.ai
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

## Build and release processes

The Bash SDK binaries are cross-compiled for `x86_64` (AKA `amd64`) and `arm64`
(e.g.: Apple Silicon M1) chipset architectures, making use of the
[musl](https://musl.libc.org/) C standard library to get self-contained
artifacts. The build logic around said artifacts lies on the GitHub workflows
within this repository; however, for contributions running `cargo clippy`,
`cargo test`, and `cargo build --release` locally is more than okay to test out
new changes.

A new release binary is cut when the `release` workflow is triggered from a
pre-existing git tag, which must match the version in the root `Cargo.toml`
file. When said pipeline successfully finishes, the resulting artifacts will be
published as a pre-release in the
[Releases](https://github.com/cto-ai/sdk-bash/releases) section. The maintainers
will manually remove the pre-release check when needed.
