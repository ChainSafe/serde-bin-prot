use serde::{Deserialize, Serialize};
use serde_bin_prot::{from_reader, to_writer};
use std::fmt::Debug;
use std::fmt::Write;

#[derive(Debug)]
pub struct TestCase<T> {
    pub input: T,
    pub expected: Vec<u8>,
}

impl<T> TestCase<T> {
    pub fn new(input: T, expected: Vec<u8>) -> Self {
        TestCase {
            input: input,
            expected: expected,
        }
    }
}

/// Prints a byte array according to the style used in the Jane Street
/// bin_prot tests. Byte array is reversed and padded up to max length with ..
/// Bytes are written in hex with lowercase letters and no 0x prefix
pub fn print_byte_array<W: Write>(w: &mut W, bytes: &[u8], max_len: usize) {
    let padding = max_len - bytes.len();
    for _ in 0..padding {
        write!(w, ".. ").unwrap();
    }

    for b in bytes.iter().rev() {
        write!(w, "{:02x} ", b).unwrap();
    }
}

pub fn roundtrip_test<'a, T: Serialize + Deserialize<'a> + PartialEq + Debug>(val: T) {
    let mut output = Vec::<u8>::new();
    to_writer(&mut output, &val).unwrap();
    let re_val: T = from_reader(output.as_slice()).unwrap();
    assert_eq!(val, re_val)
}
