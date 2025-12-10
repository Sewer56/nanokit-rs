# nanokit

[![Crates.io](https://img.shields.io/crates/v/nanokit.svg)](https://crates.io/crates/nanokit)
[![Docs.rs](https://docs.rs/nanokit/badge.svg)](https://docs.rs/nanokit)
[![CI](https://github.com/Sewer56/nanokit-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/nanokit-rs/actions)

A collection of tiny, reusable utility methods that reduce code size and improve performance.

## Features

- Fast string concatenation with minimal code size
- Bit counting utilities
- `no_std` compatible with optional `std` feature

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
nanokit = "0.2.1"
```

### String Concatenation

```rust
use nanokit::string_concat::concat_2;
let result = concat_2("Hello, ", "world!");
assert_eq!(result, "Hello, world!");
```

### Unsafe String Concatenation

Save 2 instructions when you know the result length is less than `isize::MAX`:

```rust
use nanokit::string_concat_unsafe::concat_2_no_overflow;
let result = unsafe { concat_2_no_overflow("Hello, ", "world!") };
```

### Bit Counting

```rust
use nanokit::count_bits::BitsNeeded;
assert_eq!(5u64.bits_needed_to_store(), 3); // 5 = 0b101
```

For more details, see the [nanokit crate documentation](./src/nanokit/README.MD).

## Crates

- [nanokit](./src/nanokit/README.MD): Core library with string concatenation and bit counting utilities

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Contributing

We welcome contributions! See the [Contributing Guide](https://reloaded-project.github.io/reloaded-templates-rust/manual/#contributing) for details.

## License

Licensed under [MIT](./LICENSE).
