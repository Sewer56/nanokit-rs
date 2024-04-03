/// Concatenates two strings using unsafe Rust for better performance.
///
/// This function takes two parameters, `base` and `text`, both of which can be any type
/// that implements the [`AsRef<str>`]  trait. It returns a new `String` that is the concatenation
/// of `base` and `text`.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_2;
/// let base_msg = "Hello, ";
/// let text = "world!";
/// let result = concat_2(base_msg, text);
/// assert_eq!(result, "Hello, world!");
/// ```
///
/// ```
/// use nanokit::string_concat::concat_2;
/// let base_string = String::from("The quick brown fox ");
/// let text_string = String::from("jumps over the lazy dog.");
/// let result = concat_2(base_string, text_string);
/// assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
/// ```
pub fn concat_2<S1, S2>(base: S1, text: S2) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    let base = base.as_ref();
    let text = text.as_ref();
    let total_length = base.len() + text.len();
    let mut result = String::with_capacity(total_length);

    unsafe {
        let vec = result.as_mut_vec();

        // Ensure that the vector has enough capacity
        vec.set_len(total_length);

        // Manually copy the bytes.
        core::ptr::copy_nonoverlapping(base.as_ptr(), vec.as_mut_ptr(), base.len());
        core::ptr::copy_nonoverlapping(text.as_ptr(), vec.as_mut_ptr().add(base.len()), text.len());
    }

    result
}

/// Concatenates three strings using unsafe Rust for better performance.
///
/// This function takes three parameters, `base`, `middle`, and `end`, all of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `base`, `middle`, and `end`.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_3;
/// let base = "The quick";
/// let middle = " brown fox";
/// let end = " jumps over the lazy dog.";
/// let result = concat_3(base, middle, end);
/// assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
/// ```
///
/// ```
/// use nanokit::string_concat::concat_3;
/// let base = String::from("Hello, ");
/// let middle = "beautiful ".to_string();
/// let end = String::from("world!");
/// let result = concat_3(base, middle, end);
/// assert_eq!(result, "Hello, beautiful world!");
/// ```
pub fn concat_3<S1, S2, S3>(base: S1, middle: S2, end: S3) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    S3: AsRef<str>,
{
    let base = base.as_ref();
    let middle = middle.as_ref();
    let end = end.as_ref();
    let total_length = base.len() + middle.len() + end.len();
    let mut result = String::with_capacity(total_length);

    unsafe {
        let vec = result.as_mut_vec();

        // Ensure that the vector has enough capacity
        vec.set_len(total_length);

        // Manually copy the bytes
        let mut pos = 0;
        core::ptr::copy_nonoverlapping(base.as_ptr(), vec.as_mut_ptr(), base.len());
        pos += base.len();
        core::ptr::copy_nonoverlapping(middle.as_ptr(), vec.as_mut_ptr().add(pos), middle.len());
        pos += middle.len();
        core::ptr::copy_nonoverlapping(end.as_ptr(), vec.as_mut_ptr().add(pos), end.len());
    }

    result
}

/// Concatenates four strings using unsafe Rust for better performance.
///
/// This function takes four parameters, `s1`, `s2`, `s3`, and `s4`, all of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `s1`, `s2`, `s3`, and `s4`.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_4;
/// let s1 = "The";
/// let s2 = " quick";
/// let s3 = " brown";
/// let s4 = " fox";
/// let result = concat_4(s1, s2, s3, s4);
/// assert_eq!(result, "The quick brown fox");
/// ```
///
/// ```
/// use nanokit::string_concat::concat_4;
/// let s1 = String::from("Hello");
/// let s2 = ", ".to_string();
/// let s3 = "world".to_string();
/// let s4 = "!".to_string();
/// let result = concat_4(s1, s2, s3, s4);
/// assert_eq!(result, "Hello, world!");
/// ```
pub fn concat_4<S1, S2, S3, S4>(s1: S1, s2: S2, s3: S3, s4: S4) -> String
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
    let mut result = String::with_capacity(total_length);

    unsafe {
        let vec = result.as_mut_vec();

        // Ensure that the vector has enough capacity
        vec.set_len(total_length);

        // Manually copy the bytes
        let mut pos = 0;
        core::ptr::copy_nonoverlapping(s1.as_ptr(), vec.as_mut_ptr(), s1.len());
        pos += s1.len();
        core::ptr::copy_nonoverlapping(s2.as_ptr(), vec.as_mut_ptr().add(pos), s2.len());
        pos += s2.len();
        core::ptr::copy_nonoverlapping(s3.as_ptr(), vec.as_mut_ptr().add(pos), s3.len());
        pos += s3.len();
        core::ptr::copy_nonoverlapping(s4.as_ptr(), vec.as_mut_ptr().add(pos), s4.len());
    }

    result
}

/// Concatenates five strings using unsafe Rust for better performance.
///
/// This function takes five parameters, `s1`, `s2`, `s3`, `s4`, and `s5`, all of which can be any type
/// that implements the [`AsRef<str>`] trait. It returns a new `String` that is the concatenation
/// of `s1`, `s2`, `s3`, `s4`, and `s5`.
///
/// # Examples
///
/// ```
/// use nanokit::string_concat::concat_5;
/// let s1 = "The";
/// let s2 = " quick";
/// let s3 = " brown";
/// let s4 = " fox";
/// let s5 = " jumps";
/// let result = concat_5(s1, s2, s3, s4, s5);
/// assert_eq!(result, "The quick brown fox jumps");
/// ```
///
/// ```
/// use nanokit::string_concat::concat_5;
/// let s1 = String::from("Hello");
/// let s2 = ", ".to_string();
/// let s3 = "beautiful".to_string();
/// let s4 = " world".to_string();
/// let s5 = "!".to_string();
/// let result = concat_5(s1, s2, s3, s4, s5);
/// assert_eq!(result, "Hello, beautiful world!");
/// ```
pub fn concat_5<S1, S2, S3, S4, S5>(s1: S1, s2: S2, s3: S3, s4: S4, s5: S5) -> String
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
    let mut result = String::with_capacity(total_length);

    unsafe {
        let vec = result.as_mut_vec();

        // Ensure that the vector has enough capacity
        vec.set_len(total_length);

        // Manually copy the bytes
        let mut pos = 0;
        core::ptr::copy_nonoverlapping(s1.as_ptr(), vec.as_mut_ptr(), s1.len());
        pos += s1.len();
        core::ptr::copy_nonoverlapping(s2.as_ptr(), vec.as_mut_ptr().add(pos), s2.len());
        pos += s2.len();
        core::ptr::copy_nonoverlapping(s3.as_ptr(), vec.as_mut_ptr().add(pos), s3.len());
        pos += s3.len();
        core::ptr::copy_nonoverlapping(s4.as_ptr(), vec.as_mut_ptr().add(pos), s4.len());
        pos += s4.len();
        core::ptr::copy_nonoverlapping(s5.as_ptr(), vec.as_mut_ptr().add(pos), s5.len());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_2_strings_str_slices() {
        let base = "Hello, ";
        let text = "world!";
        let result = concat_2(base, text);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_concat_2_strings_string_objects() {
        let base = String::from("The quick brown fox ");
        let text = String::from("jumps over the lazy dog.");
        let result = concat_2(base, text);
        assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
    }

    #[test]
    fn test_concat_2_strings_empty_strings() {
        let base = "";
        let text = "";
        let result = concat_2(base, text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_concat_2_strings_single_char_strings() {
        let base = "A";
        let text = "B";
        let result = concat_2(base, text);
        assert_eq!(result, "AB");
    }

    #[test]
    fn test_concat_3_str_slices() {
        let base = "The quick";
        let middle = " brown fox";
        let end = " jumps over the lazy dog.";
        let result = concat_3(base, middle, end);
        assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
    }

    #[test]
    fn test_concat_3_string_objects() {
        let base = String::from("Hello, ");
        let middle = "beautiful ".to_string();
        let end = String::from("world!");
        let result = concat_3(base, middle, end);
        assert_eq!(result, "Hello, beautiful world!");
    }

    #[test]
    fn test_concat_3_empty_strings() {
        let base = "";
        let middle = "";
        let end = "";
        let result = concat_3(base, middle, end);
        assert_eq!(result, "");
    }

    #[test]
    fn test_concat_3_single_char_strings() {
        let base = "A";
        let middle = "B";
        let end = "C";
        let result = concat_3(base, middle, end);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_concat_4_str_slices() {
        let s1 = "The";
        let s2 = " quick";
        let s3 = " brown";
        let s4 = " fox";
        let result = concat_4(s1, s2, s3, s4);
        assert_eq!(result, "The quick brown fox");
    }

    #[test]
    fn test_concat_4_string_objects() {
        let s1 = String::from("Hello");
        let s2 = ", ".to_string();
        let s3 = "world".to_string();
        let s4 = "!".to_string();
        let result = concat_4(s1, s2, s3, s4);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_concat_4_empty_strings() {
        let s1 = "";
        let s2 = "";
        let s3 = "";
        let s4 = "";
        let result = concat_4(s1, s2, s3, s4);
        assert_eq!(result, "");
    }

    #[test]
    fn test_concat_4_single_char_strings() {
        let s1 = "A";
        let s2 = "B";
        let s3 = "C";
        let s4 = "D";
        let result = concat_4(s1, s2, s3, s4);
        assert_eq!(result, "ABCD");
    }

    #[test]
    fn test_concat_5_str_slices() {
        let s1 = "The";
        let s2 = " quick";
        let s3 = " brown";
        let s4 = " fox";
        let s5 = " jumps";
        let result = concat_5(s1, s2, s3, s4, s5);
        assert_eq!(result, "The quick brown fox jumps");
    }

    #[test]
    fn test_concat_5_string_objects() {
        let s1 = String::from("Hello");
        let s2 = ", ".to_string();
        let s3 = "beautiful".to_string();
        let s4 = " world".to_string();
        let s5 = "!".to_string();
        let result = concat_5(s1, s2, s3, s4, s5);
        assert_eq!(result, "Hello, beautiful world!");
    }

    #[test]
    fn test_concat_5_empty_strings() {
        let s1 = "";
        let s2 = "";
        let s3 = "";
        let s4 = "";
        let s5 = "";
        let result = concat_5(s1, s2, s3, s4, s5);
        assert_eq!(result, "");
    }

    #[test]
    fn test_concat_5_single_char_strings() {
        let s1 = "A";
        let s2 = "B";
        let s3 = "C";
        let s4 = "D";
        let s5 = "E";
        let result = concat_5(s1, s2, s3, s4, s5);
        assert_eq!(result, "ABCDE");
    }
}
