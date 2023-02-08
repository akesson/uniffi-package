# Building

To build the example project, from the project root, run the following command:

```bash
cargo run -- --manifest-path example/rust-lib/Cargo.toml
```

This builds the `Mobile.framework` in the `example/rust-lib` directory.

To run an example of using the framework, in the `example/swift-cmd` run the following commands:

```bash
swift build
swift run
```

# Configuration

```toml
[package.metadata.swift]
# Wheter to use the nightly's option to include std-lib in the link-time optimizations
# `-Z build-std=std,panic_abort`
release-optimize-std-lib = true
```
