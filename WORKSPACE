load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    sha256 = "4a9cb4fda6ccd5b5ec393b2e944822a62e050c7c06f1ea41607f14c4fdec57a2",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.25.1/rules_rust-v0.25.1.tar.gz"],
)
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    # https://doc.rust-lang.org/edition-guide/editions/
    edition = "2021",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()


load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

crates_repository(
    name = "cargo_crate",
    cargo_lockfile = "//third_party/rust:Cargo.Bazel.lock",
    lockfile = "//third_party/rust:cargo-bazel-lock.json",
    manifests = ["//third_party/rust:Cargo.toml"],
)

load(
    "@cargo_crate//:defs.bzl",
    cargo_crate_repositories = "crate_repositories",
)

cargo_crate_repositories()
