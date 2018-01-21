# cargo-deliver


## What is it?

This will use [goreleaser](https://goreleaser.com) to deploy your Rust binaries to Github/Artifactory/Dockerhub.
It's still experimental and will only work once/if [Rust support gets added to goreleaser](https://github.com/goreleaser/goreleaser/issues/496#issuecomment-359209326).


## Installation

```
cargo install cargo-deliver
```

To upgrade:

```
cargo install --force cargo-deliver
```

Or clone and build with `$ cargo build --release` then place in your $PATH.


## Usage

1. Add a `.goreleaser.yml` to your project. Here is an example:

```yaml
rust:
- target:
  - x86_64-apple-darwin
  binary: cargo-deliver

# Archive customization
archive:
  format: tar.gz
  replacements:
    amd64: 64-bit
    darwin: macOS
    linux: Linux
```

This will build your Rust application as a static binary for macOS and as a `tar.gz` archive.  
For all possible options, see the [goreleaser docs](http://goreleaser.com/).  

You can get a list of all possible targets with 

```
rustc --print target-list
```


2. Tag your current commit, e.g.

```
git tag -a "v0.1.0"
```

3. If you want to deploy to Github, you need to export a `GITHUB_TOKEN` environment variable, which should contain a GitHub token with the repo scope selected. It will be used to deploy releases to your GitHub repository. Create a token [here](https://github.com/settings/tokens/new).

4. Run the following command

```
cargo deliver
```


## Similar tools

* [cargo-docker](https://github.com/DenisKolodin/cargo-docker) - A cargo subcommand to build Rust code inside a Docker and get the result back.
* [cargo-hublish](https://github.com/chasinglogic/cargo-hublish) - A cargo subcommand for publishing Rust projects to Github (not supporting different build targets).

