use serde::{Deserialize, Serialize};
use serde_bin_prot::{from_reader, to_writer};
use std::fmt::Debug;
use std::io::Write;
use std::fs::File;
use serde_bin_prot::WriteBinProtExt;

/// Prints a byte array according to the style used in the Jane Street
/// bin_prot tests. Byte array is reversed and padded up to max length with ..
/// Bytes are written in hex with lowercase letters and no 0x prefix
pub fn write_byte_array<W: std::fmt::Write>(w: &mut W, bytes: &[u8], max_len: usize) {
    let padding = max_len - bytes.len();
    for _ in 0..padding {
        write!(w, ".. ").unwrap();
    }

    for b in bytes.iter().rev() {
        write!(w, "{:02x} ", b).unwrap();
    }
}

/// Prints a byte array according to the style used in the Jane Street
/// bin_prot tests. Byte array is reversed and padded up to max length with ..
/// Bytes are written in hex with lowercase letters and no 0x prefix
pub fn write_integer_test_repr(num: i64, max_len: usize, f: &mut File) {
    let mut bytes:Vec<u8> = vec![];
    bytes.bin_write_integer(num).unwrap();
    
    let padding = max_len - bytes.len();
    for _ in 0..padding {
        f.write(".. ".as_bytes()).unwrap();
    }
    for i in bytes.iter().rev() {
        let fmt = format!("{:02x} ", i);
        f.write(fmt.as_bytes()).unwrap();
    }
    
    f.write(format!("-> {}\n", num).as_bytes()).unwrap();
}

pub fn roundtrip_test<'a, T: Serialize + Deserialize<'a> + PartialEq + Debug>(val: T) {
    let mut output = Vec::<u8>::new();
    to_writer(&mut output, &val).unwrap();
    let re_val: T = from_reader(output.as_slice()).unwrap();
    assert_eq!(val, re_val)
}
