use serde::{Deserialize, Serialize};
use serde_bin_prot::{from_reader, to_writer};
use std::fmt::Debug;
use std::io::Write;
use std::fs::File;
use serde_bin_prot::WriteBinProtExt;

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
pub(crate) struct B2 {
    pub(crate) y: BInner3,
    pub(crate) z: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct B3 {
    pub(crate) y: BInner2,
    pub(crate) z: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct BInner {
    pub(crate) w: i64,
    pub(crate) x: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum BInner2 {
    V0(BInner),
    V1(BInner4),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct BInner3 {
    pub(crate) dummy: i64,
    pub(crate) inner: BInner2,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum BInner4 {
    V0(BInner),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct C {
    pub(crate) y: CInner,
    pub(crate) z: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum CInner2 {
    V0,
    V1(()),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum CInner {
    V0,
    V1(CInner2),
}

pub(crate) struct Outer {
    pub(crate) y: Inner,
    pub(crate) z: Option<i64>,
}

pub(crate) struct Inner {
    pub(crate) w: i64,
    pub(crate) x: i64,
}

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
