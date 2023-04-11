`cargo-emanate`
===============

Cargo subcommand for publishing workspace-based crates and checking dependency versions.

Created with simplicity and manual management in mind.

## Features

### Workspaces
The following commands are available:
- `version`: Applies the specified version to the workspace and its member crates.
- `publish`: Publish all crates in a hierarchial dependency order.
- `check`: Scans all dependencies in the crate and checks them against crates.io outputing the difference to console. You can use this information to manually update dependencies.

Required project structure:
- Versions of all member crates must be linked to the workspace using `version.workspace = true` in the `Cargo.toml`
- Versions of all member crates will always match the workspace version
- Crates that should be ignored should have `[package] publish = false` properties enabled

If you require to publish a crate within your workspace with a specific version, you should manually change the version settings and publish it. This tool currently does not track versions within workspace crates.

### Crates
The following commands are available:
- `check`: Scans all dependencies in the crate and checks them against crates.io outputing the difference to console. You can use this information to manually update dependencies.

### General
NOTE: This tool supports only fixed version use in the workspace. i.e. dependency versions like "^2.0.0" or "1.0" will be rejected.  Versions must always comply with the exact `x.y.z[-suffix]` schema. This is done to prevent a potential of code injections during minor and patch dependency releases.

