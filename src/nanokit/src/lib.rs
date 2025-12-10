#![no_std]
#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

pub mod count_bits;
#[cfg(feature = "c-exports")]
pub mod exports;
pub mod string_concat;
pub mod string_concat_unsafe;
