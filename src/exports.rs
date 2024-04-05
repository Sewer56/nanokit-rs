#![allow(improper_ctypes_definitions)]

// Exports for code size measure only.
use crate::{string_concat::*, string_concat_unsafe::*};

#[no_mangle]
pub extern "C" fn concat_2_c(base: &str, text: &str) -> String {
    concat_2(base, text)
}

#[no_mangle]
pub extern "C" fn concat_3_c(base: &str, middle: &str, end: &str) -> String {
    concat_3(base, middle, end)
}

#[no_mangle]
pub extern "C" fn concat_4_c(s1: &str, s2: &str, s3: &str, s4: &str) -> String {
    concat_4(s1, s2, s3, s4)
}

#[no_mangle]
pub extern "C" fn concat_5_c(s1: &str, s2: &str, s3: &str, s4: &str, s5: &str) -> String {
    concat_5(s1, s2, s3, s4, s5)
}

#[no_mangle]
pub extern "C" fn concat_2_no_overflow_c(base: &str, text: &str) -> String {
    unsafe { concat_2_no_overflow(base, text) }
}
