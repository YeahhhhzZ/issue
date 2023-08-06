# Usage
## Add rust dependencies
Add your rust dependencies to the Cargo.toml, and then run
```shell
CARGO_BAZEL_REPIN=true bazel sync --only=cargo_crate
```