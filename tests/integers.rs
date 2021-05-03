///
/// These tests aim to follow the Jane Street OCaml tests as closely as possible
/// The tests contain a lookup table for auto-generated integer encoding test cases
/// Each tests case is expressed as a single line as follows
/// ```
/// <type>| b8 b7 b6 b5 b4 b3 b2 b1 b0 -> <integer-repr>
/// ```
/// Note the bytes are little-endian encoded. The bytes may be written as '..'
/// which is a placeholder (e.g. no byte)
///
/// These tests can be parsed and executed directly from their source

mod integers_repr_tests_64bit;

use core::convert::TryFrom;
use serde_bin_prot::{to_writer, Nat0};
use integers_repr_tests_64bit::{get_test_cases, OcamlIntegerType};

#[derive(Debug)]
enum OCamlInteger {
    Int(i64),
    Nat0(Nat0),
}


impl TryFrom<(OcamlIntegerType, i64)> for OCamlInteger {
    type Error = String;
    fn try_from(i: (OcamlIntegerType, i64)) -> Result<Self, Self::Error> {
        match i.0 {
        OcamlIntegerType::int
            | OcamlIntegerType::int32
            | OcamlIntegerType::int64
            | OcamlIntegerType::int_16bit
            | OcamlIntegerType::int_32bit
            | OcamlIntegerType::int_64bit
            | OcamlIntegerType::int64_bits
            | OcamlIntegerType::network16_int
            | OcamlIntegerType::network32_int
            | OcamlIntegerType::network64_int
            | OcamlIntegerType::network32_int32
            | OcamlIntegerType::network64_int64 => {
                Ok(Self::Int(i.1))
            },
            OcamlIntegerType::nat0 => {
                let v = u64::try_from(i.1).map_err(|_| "Tryed to parse signed int to nat0")?;
                Ok(Self::Nat0(Nat0::new(v)))
            },
            OcamlIntegerType::variant_int => Err("Not supported".to_string()),
        }
    }
}

#[derive(Debug)]
struct IntegerTestCase {
    pub bytes: Vec<u8>,
    pub integer: OCamlInteger,
}

impl TryFrom<(OcamlIntegerType, Vec<u8>, i64)> for IntegerTestCase {
    type Error = String;
    fn try_from(i: (OcamlIntegerType, Vec<u8>, i64)) -> Result<Self, Self::Error> {
        Ok(Self {
            bytes: i.1,
            integer: OCamlInteger::try_from((i.0, i.2))?
        })
    }
}

impl IntegerTestCase {
    /// Test that serializing the integer as the given type 
    /// yields the correct bytes
    pub fn test_ser(&self) {
        let mut output = Vec::<u8>::new();
        match &self.integer {
            OCamlInteger::Int(v) => { to_writer(&mut output, &v).unwrap(); }
            OCamlInteger::Nat0(v) => { to_writer(&mut output, &v).unwrap(); }
        }
        output.reverse(); // Not 100% sure why this is required. Seek confirmation.
        assert_eq!(output, self.bytes)
    }
}

#[test]
fn test_serialize_integers() {
    for case_tuple in get_test_cases() {
        println!("{:?}", case_tuple);
        IntegerTestCase::try_from(case_tuple).unwrap().test_ser();
    }
}
