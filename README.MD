# nanokit

[![Crates.io](https://img.shields.io/crates/v/nanokit.svg)](https://crates.io/crates/nanokit)
[![Docs.rs](https://docs.rs/nanokit/badge.svg)](https://docs.rs/nanokit)
[![CI](https://github.com/Sewer56/nanokit-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/nanokit-rs/actions)

## About

A collection of tiny, reusable utility methods that reduce code size and improve performance.
Readme for developers/contributors is available at [README-DEV.MD](./README-DEV.MD).

## String Construction

Fast way to concatenate strings:

```rust
use nanokit::string_concat::concat_2;
let base_msg = "Hello, ";
let text = "world!";
let result = concat_2(base_msg, text);
assert_eq!(result, "Hello, world!");
```

And with any type that implements `AsRef<str>`:

```rust
use nanokit::string_concat::concat_2;
let base_string = String::from("The quick brown fox ");
let text_string = String::from("jumps over the lazy dog.");
let result = concat_2(base_string, text_string);
assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
```

This is similar to libraries like [concat_strs][concat-strs] and [string_concat][string-concat], except that instead of pushing strings onto a preallocated `String` instance, we instead unsafely create a new `String` instance and adjust the length.

This saves around `150 bytes` of code in codebases that otherwise don't use string concatenation (`push_str` or add).
Also saves some instructions.

Additional methods `concat_3`, `concat_4`, `concat_5` exist.

### Unsafe Concat

You can save on another 2 instructions per concatenation if you know the final string length
is lesser than `isize::MAX`.

```rust
use nanokit::string_concat::concat_2;
let base_msg = "Hello, ";
let text = "world!";
let result = concat_2_no_overflow(base_msg, text);
assert_eq!(result, "Hello, world!");
```

### Features

- `no-inline-concat`: Disables inlining of string concat functions (saves code size).

## Numeric Utilities

### Count Needed Bits

Provides a method to determine the number of bits needed to store a given number.

```rust
let number: u64 = 5;
println!("Bits needed for {}: {}", number, number.bits_needed_to_store());
```

The produced code does not panic, nor does it branch.
Generated x86_64 assembly for `u64`:

```asm
bits_needed_to_store:
    lzcnt   rcx, rdi
    mov     eax, 64
    sub     eax, ecx
    ret
```

Note: The number of bits needed to store the value `0` is returned as `0`.

This may not match your expectations. If you expect the result to be 1; try
the [count-digits] crate by [nordzilla]. Do note that said crate uses `ilog2`, which
may panic.

## Related Crates

- [itoa](https://crates.io/crates/itoa): Integer to text.

## Contributing

See [CONTRIBUTING](CONTRIBUTING.MD) for guidance on how to contribute to this project.  

## License

`nanokit` is part of the `Reloaded` suite of libraries.
Licensed under [MIT](./LICENSE).

If you find this library useful, please contribute back!!

<!-- Links -->
[concat-strs]: https://crates.io/crates/concat_strs
[string-concat]: https://crates.io/crates/string_concat
[count-digits]: https://github.com/nordzilla/count-digits
[nordzilla]: https://github.com/nordzilla