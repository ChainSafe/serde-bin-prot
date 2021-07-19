use serde::{Deserialize, Serialize};
use serde_bin_prot::{from_reader, to_writer};
use std::fmt::Debug;
use std::fmt::Write;

/// Macro that lets one use jane street's bin prot expect test cases blocks
/// and assert using serde-bin-prot's APIs.
#[macro_export]
macro_rules! bin_prot_test {
    ($($(..) * $($expected:literal) * -> $typ:expr),*)  => {
        $(
        let mut output = vec![];
        serde_bin_prot::to_writer(&mut output, &$typ).expect("Failed writing bin-prot encoded data");
        assert_eq!(vec![$($expected,)*], output.into_iter().rev().collect::<Vec<u8>>());
        )*
    };
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct A {
   pub(crate) x: i64,
   pub(crate) y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct B {
    pub(crate) y: BInner,
    pub(crate) z: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct BInner {
    pub(crate)  w: i64,
    pub(crate) x: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct C {
    pub(crate) y: CInner,
    pub(crate) z: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct CInner {}

pub(crate) struct Outer {
    pub(crate) y: Inner,
    pub(crate) z: Option<i64>
}

pub(crate) struct Inner {
    pub(crate) w: i64,
    pub(crate) x: i64,
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
