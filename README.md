# cargo-ship


## What is it?

This will use [goreleaser](https://goreleaser.com) to deploy your Rust binaries to Github/Artifactory/Dockerhub.
It's still experimental.


## Installation

```
cargo install cargo-ship
```

To upgrade:

```
cargo install --force cargo-ship
```

Or clone and build with `$ cargo build --release` then place in your $PATH.


## Usage

1. Add a `.goreleaser.yml` to your project. Here is an example:

```yaml
rust:
- target:
  - x86_64-apple-darwin
  binary: cargo-ship

# Archive customization
archive:
  format: tar.gz
  replacements:
    amd64: 64-bit
    darwin: macOS
    linux: Linux
```

2. Tag your current commit, e.g.

```
git tag -a "v0.1.0"
```

3. Next, you need to export a `GITHUB_TOKEN` environment variable, which should contain a GitHub token with the repo scope selected. It will be used to deploy releases to your GitHub repository. Create a token [here](https://github.com/settings/tokens/new).

4. Run the following command

```
cargo ship
```
