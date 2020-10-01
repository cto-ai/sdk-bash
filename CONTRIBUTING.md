# Project Structure and Release Process

## Project Structure

The `bash-sdk` project contains three Rust crates, which are not
combined into a Cargo workspace.

The base `bash-sdk` crate contains only the frontend programs `ctoai`,
`sdk`, and `ux`. These use the `clap` library (https://lib.rs/clap) to
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

## Release Process

The Bash SDK is automatically built into a Docker base image by the
GitLab CI whenever there is a merge to the `master` branch. Code can
only be merged to master with a version bump, using the version in the
root `Cargo.toml` file as the version for the SDK as a whole.

Unlike the other SDKs, the daemon version in the image is fixed for
each version of the Bash SDK. If new daemon changes need to be
propagated to the Bash SDK image, a new version must be created. This
commit only needs to increment the version number in the Cargo.toml
and requires no other changes.

The image is tagged with tags `<major>-buster-slim`,
`<major>.<minor>-buster-slim`, and
`<major>.<minor>.<patch>-buster-slim`, similar to how the daemon
images are tagged.
