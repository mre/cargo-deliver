# cargo-ship


## What is it?

This will use [goreleaser](https://goreleaser.com) to deploy your Rust binaries to Github/Artifactory/Dockerhub.
It's still experimental.


## Usage

Add a `.goreleaser.yml` to your project. Here is an example:

```
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

After that run the following command

```
cargo ship
```
