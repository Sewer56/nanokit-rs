// src/lib.rs

/// A trait to calculate the minimum number of bits required to store a number.
pub trait BitsNeeded {
    /// Returns the minimum number of bits required to store the number.
    ///
    /// This returns the bits needed to store a given number.
    /// Examples:
    ///
    /// - 0: 0 bits
    /// - 1: 1 bit (0b1)
    /// - 2: 2 bits (0b10)
    /// - 3: 2 bits (0b11)
    ///  - ...
    /// - 1023: 10 bits (0b1111111111)
    /// - 1024: 11 bits (0b10000000000)
    ///
    fn bits_needed_to_store(&self) -> u32;
}

/// Macro to implement the `BitsNeeded` trait for multiple numeric types.
///
/// # Parameters
///
/// * `$type`: The numeric type (e.g., `u8`, `u16`, `i32`, etc.).
/// * `$bits`: The total number of bits for the type (e.g., `8` for `u8`, `16` for `u16`, etc.).
#[macro_export]
macro_rules! impl_bits_needed {
    ($($t:ty => $bits:expr),* $(,)?) => {
        $(
            impl BitsNeeded for $t {

                fn bits_needed_to_store(&self) -> u32 {
                    $bits - self.leading_zeros()
                }
            }
        )*
    }
}

// Implement `BitsNeeded` for all unsigned integer types
impl_bits_needed! {
    u8 => u8::BITS,
    u16 => u16::BITS,
    u32 => u32::BITS,
    u64 => u64::BITS,
    u128 => u128::BITS,
    usize => usize::BITS,
}

// Implement `BitsNeeded` for all signed integer types
impl_bits_needed! {
    i8 => i8::BITS,
    i16 => i16::BITS,
    i32 => i32::BITS,
    i64 => i64::BITS,
    i128 => i128::BITS,
    isize => isize::BITS,
}

#[cfg(test)]
mod tests {
    use super::BitsNeeded;
    use rstest::rstest;

    #[rstest]
    #[case(0u8, 0)]
    #[case(1u8, 1)]
    #[case(2u8, 2)]
    #[case(3u8, 2)]
    #[case(4u8, 3)]
    #[case(5u8, 3)]
    #[case(7u8, 3)]
    #[case(8u8, 4)]
    #[case(15u8, 4)]
    #[case(16u8, 5)]
    #[case(31u8, 5)]
    #[case(32u8, 6)]
    #[case(63u8, 6)]
    #[case(64u8, 7)]
    #[case(127u8, 7)]
    #[case(128u8, 8)]
    #[case(255u8, 8)]
    fn test_bits_needed_u8(#[case] input: u8, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (u8)"
        );
    }

    #[rstest]
    #[case(0u16, 0)]
    #[case(1u16, 1)]
    #[case(2u16, 2)]
    #[case(3u16, 2)]
    #[case(4u16, 3)]
    #[case(5u16, 3)]
    #[case(7u16, 3)]
    #[case(8u16, 4)]
    #[case(15u16, 4)]
    #[case(16u16, 5)]
    #[case(31u16, 5)]
    #[case(32u16, 6)]
    #[case(63u16, 6)]
    #[case(64u16, 7)]
    #[case(127u16, 7)]
    #[case(128u16, 8)]
    #[case(255u16, 8)]
    #[case(256u16, 9)]
    #[case(1023u16, 10)]
    #[case(1024u16, 11)]
    #[case(u16::MAX, 16)]
    fn test_bits_needed_u16(#[case] input: u16, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (u16)"
        );
    }

    #[rstest]
    #[case(0u32, 0)]
    #[case(1u32, 1)]
    #[case(2u32, 2)]
    #[case(3u32, 2)]
    #[case(4u32, 3)]
    #[case(5u32, 3)]
    #[case(7u32, 3)]
    #[case(8u32, 4)]
    #[case(15u32, 4)]
    #[case(16u32, 5)]
    #[case(31u32, 5)]
    #[case(32u32, 6)]
    #[case(63u32, 6)]
    #[case(64u32, 7)]
    #[case(127u32, 7)]
    #[case(128u32, 8)]
    #[case(255u32, 8)]
    #[case(256u32, 9)]
    #[case(1023u32, 10)]
    #[case(1024u32, 11)]
    #[case(u32::MAX, 32)]
    fn test_bits_needed_u32(#[case] input: u32, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (u32)"
        );
    }

    #[rstest]
    #[case(0u64, 0)]
    #[case(1u64, 1)]
    #[case(2u64, 2)]
    #[case(3u64, 2)]
    #[case(4u64, 3)]
    #[case(5u64, 3)]
    #[case(7u64, 3)]
    #[case(8u64, 4)]
    #[case(15u64, 4)]
    #[case(16u64, 5)]
    #[case(31u64, 5)]
    #[case(32u64, 6)]
    #[case(63u64, 6)]
    #[case(64u64, 7)]
    #[case(127u64, 7)]
    #[case(128u64, 8)]
    #[case(255u64, 8)]
    #[case(256u64, 9)]
    #[case(1023u64, 10)]
    #[case(1024u64, 11)]
    #[case(u64::MAX, 64)]
    fn test_bits_needed_u64(#[case] input: u64, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (u64)"
        );
    }

    #[rstest]
    #[case(0u128, 0)]
    #[case(1u128, 1)]
    #[case(2u128, 2)]
    #[case(3u128, 2)]
    #[case(4u128, 3)]
    #[case(5u128, 3)]
    #[case(7u128, 3)]
    #[case(8u128, 4)]
    #[case(15u128, 4)]
    #[case(16u128, 5)]
    #[case(31u128, 5)]
    #[case(32u128, 6)]
    #[case(63u128, 6)]
    #[case(64u128, 7)]
    #[case(127u128, 7)]
    #[case(128u128, 8)]
    #[case(255u128, 8)]
    #[case(256u128, 9)]
    #[case(1023u128, 10)]
    #[case(1024u128, 11)]
    #[case(u128::MAX, 128)]
    fn test_bits_needed_u128(#[case] input: u128, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (u128)"
        );
    }

    #[rstest]
    #[case(0usize, 0)]
    #[case(1usize, 1)]
    #[case(2usize, 2)]
    #[case(3usize, 2)]
    #[case(4usize, 3)]
    #[case(5usize, 3)]
    #[case(7usize, 3)]
    #[case(8usize, 4)]
    #[case(15usize, 4)]
    #[case(16usize, 5)]
    #[case(31usize, 5)]
    #[case(32usize, 6)]
    #[case(63usize, 6)]
    #[case(64usize, 7)]
    #[case(127usize, 7)]
    #[case(128usize, 8)]
    #[case(255usize, 8)]
    #[case(256usize, 9)]
    #[case(1023usize, 10)]
    #[case(1024usize, 11)]
    #[case(usize::MAX, (core::mem::size_of::<usize>() * 8) as u32)]
    fn test_bits_needed_usize(#[case] input: usize, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (usize)",
        );
    }

    #[rstest]
    #[case(0i8, 0)]
    #[case(1i8, 1)]
    #[case(-1i8, 8)] // Two's complement: 11111111, leading zeros: 0
    #[case(2i8, 2)]
    #[case(-2i8, 8)] // Two's complement: 11111110, leading zeros: 0
    #[case(3i8, 2)]
    #[case(-3i8, 8)] // Two's complement: 11111101, leading zeros: 0
    #[case(4i8, 3)]
    #[case(7i8, 3)]
    #[case(8i8, 4)]
    #[case(15i8, 4)]
    #[case(16i8, 5)] // Note: i8 max is 127, 16 exceeds i8 range
    #[case(i8::MAX, 7)]
    #[case(i8::MIN, 8)]
    fn test_bits_needed_i8(#[case] input: i8, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (i8)"
        );
    }

    #[rstest]
    #[case(0i16, 0)]
    #[case(1i16, 1)]
    #[case(-1i16, 16)] // Two's complement: 1111111111111111, leading zeros: 0
    #[case(2i16, 2)]
    #[case(-2i16, 16)] // Two's complement: 1111111111111110, leading zeros: 0
    #[case(3i16, 2)]
    #[case(-3i16, 16)] // Two's complement: 1111111111111101, leading zeros: 0
    #[case(4i16, 3)]
    #[case(7i16, 3)]
    #[case(8i16, 4)]
    #[case(15i16, 4)]
    #[case(16i16, 5)]
    #[case(31i16, 5)]
    #[case(32i16, 6)]
    #[case(63i16, 6)]
    #[case(64i16, 7)]
    #[case(127i16, 7)]
    #[case(128i16, 8)]
    #[case(255i16, 8)]
    #[case(256i16, 9)]
    #[case(1023i16, 10)]
    #[case(1024i16, 11)]
    #[case(i16::MAX, 15)]
    #[case(i16::MIN, 16)]
    fn test_bits_needed_i16(#[case] input: i16, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (i16)"
        );
    }

    #[rstest]
    #[case(0i32, 0)]
    #[case(1i32, 1)]
    #[case(-1i32, 32)] // Two's complement: 32 leading bits
    #[case(2i32, 2)]
    #[case(-2i32, 32)]
    #[case(3i32, 2)]
    #[case(-3i32, 32)]
    #[case(4i32, 3)]
    #[case(7i32, 3)]
    #[case(8i32, 4)]
    #[case(15i32, 4)]
    #[case(16i32, 5)]
    #[case(31i32, 5)]
    #[case(32i32, 6)]
    #[case(63i32, 6)]
    #[case(64i32, 7)]
    #[case(127i32, 7)]
    #[case(128i32, 8)]
    #[case(255i32, 8)]
    #[case(256i32, 9)]
    #[case(1023i32, 10)]
    #[case(1024i32, 11)]
    #[case(i32::MAX, 31)]
    #[case(i32::MIN, 32)]
    fn test_bits_needed_i32(#[case] input: i32, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (i32)"
        );
    }

    #[rstest]
    #[case(0i64, 0)]
    #[case(1i64, 1)]
    #[case(-1i64, 64)] // Two's complement: 64 leading bits
    #[case(2i64, 2)]
    #[case(-2i64, 64)]
    #[case(3i64, 2)]
    #[case(-3i64, 64)]
    #[case(4i64, 3)]
    #[case(7i64, 3)]
    #[case(8i64, 4)]
    #[case(15i64, 4)]
    #[case(16i64, 5)]
    #[case(31i64, 5)]
    #[case(32i64, 6)]
    #[case(63i64, 6)]
    #[case(64i64, 7)]
    #[case(127i64, 7)]
    #[case(128i64, 8)]
    #[case(255i64, 8)]
    #[case(256i64, 9)]
    #[case(1023i64, 10)]
    #[case(1024i64, 11)]
    #[case(i64::MAX, 63)]
    #[case(i64::MIN, 64)]
    fn test_bits_needed_i64(#[case] input: i64, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (i64)"
        );
    }

    #[rstest]
    #[case(0i128, 0)]
    #[case(1i128, 1)]
    #[case(-1i128, 128)] // Two's complement: 128 leading bits
    #[case(2i128, 2)]
    #[case(-2i128, 128)]
    #[case(3i128, 2)]
    #[case(-3i128, 128)]
    #[case(4i128, 3)]
    #[case(7i128, 3)]
    #[case(8i128, 4)]
    #[case(15i128, 4)]
    #[case(16i128, 5)]
    #[case(31i128, 5)]
    #[case(32i128, 6)]
    #[case(63i128, 6)]
    #[case(64i128, 7)]
    #[case(127i128, 7)]
    #[case(128i128, 8)]
    #[case(255i128, 8)]
    #[case(256i128, 9)]
    #[case(1023i128, 10)]
    #[case(1024i128, 11)]
    #[case(i128::MAX, 127)]
    #[case(i128::MIN, 128)]
    fn test_bits_needed_i128(#[case] input: i128, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (i128)"
        );
    }

    #[rstest]
    #[case(0isize, 0)]
    #[case(1isize, 1)]
    #[case(-1isize, (core::mem::size_of::<isize>() * 8) as u32)]
    #[case(2isize, 2)]
    #[case(-2isize, (core::mem::size_of::<isize>() * 8) as u32)]
    #[case(3isize, 2)]
    #[case(-3isize, (core::mem::size_of::<isize>() * 8) as u32)]
    #[case(4isize, 3)]
    #[case(7isize, 3)]
    #[case(8isize, 4)]
    #[case(15isize, 4)]
    #[case(16isize, 5)]
    #[case(31isize, 5)]
    #[case(32isize, 6)]
    #[case(63isize, 6)]
    #[case(64isize, 7)]
    #[case(127isize, 7)]
    #[case(128isize, 8)]
    #[case(255isize, 8)]
    #[case(256isize, 9)]
    #[case(1023isize, 10)]
    #[case(1024isize, 11)]
    #[case(isize::MAX, ((core::mem::size_of::<isize>() * 8) as u32) - 1)]
    #[case(isize::MIN, (core::mem::size_of::<isize>() * 8) as u32)]
    fn test_bits_needed_isize(#[case] input: isize, #[case] expected: u32) {
        assert_eq!(
            input.bits_needed_to_store(),
            expected,
            "Failed for input: {input} (isize)"
        );
    }
}
