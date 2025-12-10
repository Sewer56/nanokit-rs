# nanokit

[![Crates.io](https://img.shields.io/crates/v/nanokit.svg)](https://crates.io/crates/nanokit)
[![Docs.rs](https://docs.rs/nanokit/badge.svg)](https://docs.rs/nanokit)
[![CI](https://github.com/Sewer56/nanokit-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/nanokit-rs/actions)

A collection of tiny, reusable utility methods that reduce code size and improve performance.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
nanokit = "0.2.1"
```

```rust
use nanokit::string_concat::concat_2;
let result = concat_2("Hello, ", "world!");
assert_eq!(result, "Hello, world!");
```

See the [nanokit crate documentation](./src/nanokit/README.MD) for detailed usage.

## Crates

- [nanokit](./src/nanokit/README.MD): Core library with string concatenation and bit counting utilities

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Contributing

See [CONTRIBUTING](CONTRIBUTING.MD) for guidance on how to contribute to this project.

## License

Licensed under [MIT](./LICENSE).