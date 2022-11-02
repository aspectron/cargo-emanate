cargo emanate
=============

[![autocfg crate](https://img.shields.io/crates/v/autocfg.svg)](https://crates.io/crates/autocfg)

Cargo subcommand for multi-repository management and creation of application packages.

*This crate is under development.*

## Installation

`cargo install cargo-emanate`

## Usage

* `cargo emanate sync` - clones or pulls files in the `Emanate.toml` file
* `cargo emanate build` - builds configured packages
* `cargo emanate purge --force` - deletes all repositories present in `Emanate.toml`

## Configuration

`Emanate.toml` format:

### Repositoy declarations

The emanator configuration file can container multiple repository references as follows:
```
[[repository]]
url = "https://github.com/snapview/tungstenite-rs"
```
This denotes a repository declaration. When running `cargo emanate sync`, emanator will deploy these repositories in the local folder.  It will run `git clone` if the repository is absent or `git pull` if the repository is present.

### Build declarations
```
[[build]]
cmd = "bash build-web"
folder = "repo-path/wasm"
```
This denotes a build declaration. `cargo emanate build` will run all `build` declarations (currently these declarations are executed sequentially).

