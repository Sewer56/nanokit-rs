use crate::string_concat::*;
use core::hint::unreachable_unchecked;

/// Concatenates two strings using unsafe Rust for better performance.
///
/// This function takes two parameters, `base` and `text`, both of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `base` and `text`.
///
/// # Safety
///
/// This function should only be used when the combined length of `base` and `text` does not exceed `isize::MAX`.
/// Using this function with strings that exceed the maximum length will result in undefined behavior.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_2_no_overflow;
/// let base_msg = "Hello, ";
/// let text = "world!";
/// let result = unsafe { concat_2_no_overflow(base_msg, text) };
/// assert_eq!(result, "Hello, world!");
/// ```
pub unsafe fn concat_2_no_overflow<S1, S2>(base: S1, text: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let base = base.as_ref();
    let text = text.as_ref();
    let total_length = base.len() + text.len();

    if total_length > isize::MAX as usize {
        unsafe { unreachable_unchecked() };
    }

    concat_2(base, text)
}

/// Concatenates three strings using unsafe Rust for better performance.
///
/// This function takes three parameters, `base`, `middle`, and `end`, all of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `base`, `middle`, and `end`.
///
/// # Safety
///
/// This function should only be used when the combined length of `base`, `middle`, and `end` does not exceed `isize::MAX`.
/// Using this function with strings that exceed the maximum length will result in undefined behavior.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_3_no_overflow;
/// let base = "The quick";
/// let middle = " brown fox";
/// let end = " jumps over the lazy dog.";
/// let result = unsafe { concat_3_no_overflow(base, middle, end) };
/// assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
/// ```
pub unsafe fn concat_3_no_overflow<S1, S2, S3>(base: S1, middle: S2, end: S3) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    let base = base.as_ref();
    let middle = middle.as_ref();
    let end = end.as_ref();
    let total_length = base.len() + middle.len() + end.len();

    if total_length > isize::MAX as usize {
        unsafe { unreachable_unchecked() };
    }

    concat_3(base, middle, end)
}

/// Concatenates four strings using unsafe Rust for better performance.
///
/// This function takes four parameters, `s1`, `s2`, `s3`, and `s4`, all of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `s1`, `s2`, `s3`, and `s4`.
///
/// # Safety
///
/// This function should only be used when the combined length of `s1`, `s2`, `s3`, and `s4` does not exceed `isize::MAX`.
/// Using this function with strings that exceed the maximum length will result in undefined behavior.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_4_no_overflow;
/// let s1 = "The";
/// let s2 = " quick";
/// let s3 = " brown";
/// let s4 = " fox";
/// let result = unsafe { concat_4_no_overflow(s1, s2, s3, s4) };
/// assert_eq!(result, "The quick brown fox");
/// ```
pub unsafe fn concat_4_no_overflow<S1, S2, S3, S4>(s1: S1, s2: S2, s3: S3, s4: S4) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
    S4: AsRef<str>,
{
    let s1 = s1.as_ref();
    let s2 = s2.as_ref();
    let s3 = s3.as_ref();
    let s4 = s4.as_ref();
    let total_length = s1.len() + s2.len() + s3.len() + s4.len();

    if total_length > isize::MAX as usize {
        unsafe { unreachable_unchecked() };
    }

    concat_4(s1, s2, s3, s4)
}

/// Concatenates five strings using unsafe Rust for better performance.
///
/// This function takes five parameters, `s1`, `s2`, `s3`, `s4`, and `s5`, all of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `s1`, `s2`, `s3`, `s4`, and `s5`.
///
/// # Safety
///
/// This function should only be used when the combined length of `s1`, `s2`, `s3`, `s4`, and `s5` does not exceed `isize::MAX`.
/// Using this function with strings that exceed the maximum length will result in undefined behavior.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_5_no_overflow;
/// let s1 = "The";
/// let s2 = " quick";
/// let s3 = " brown";
/// let s4 = " fox";
/// let s5 = " jumps";
/// let result = unsafe { concat_5_no_overflow(s1, s2, s3, s4, s5) };
/// assert_eq!(result, "The quick brown fox jumps");
/// ```
pub unsafe fn concat_5_no_overflow<S1, S2, S3, S4, S5>(
    s1: S1,
    s2: S2,
    s3: S3,
    s4: S4,
    s5: S5,
) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
    S4: AsRef<str>,
    S5: AsRef<str>,
{
    let s1 = s1.as_ref();
    let s2 = s2.as_ref();
    let s3 = s3.as_ref();
    let s4 = s4.as_ref();
    let s5 = s5.as_ref();
    let total_length = s1.len() + s2.len() + s3.len() + s4.len() + s5.len();

    if total_length > isize::MAX as usize {
        unsafe { unreachable_unchecked() };
    }

    concat_5(s1, s2, s3, s4, s5)
}
