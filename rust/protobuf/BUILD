load("@rules_rust//proto/protobuf:toolchain.bzl", "PROTO_COMPILE_DEPS")
load("@rules_rust//proto/protobuf:defs.bzl", "rust_proto_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
package(default_visibility = ["//visibility:public"])

proto_library(
    name = "example_proto",
    srcs = ["example.proto"],
)

rust_proto_library(
    name = "example_proto_rust",
    deps = [
        ":example_proto",
    ]
)

rust_binary(
    name = "main",
    srcs = [
        "main.rs",
    ],
    deps = [
        ":example_proto_rust",
    # ]
    ] + PROTO_COMPILE_DEPS,
)
