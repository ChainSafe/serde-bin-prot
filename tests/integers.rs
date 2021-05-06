///
/// These tests aim to follow the Jane Street OCaml tests as closely as possible
/// The tests contain a lookup table for auto-generated integer encoding test cases
///
/// Note the bytes are little-endian encoded.
///
/// These tests can be parsed and executed directly from their source
mod integers_repr_tests_64bit_smol;

use core::convert::TryFrom;
use integers_repr_tests_64bit_smol::{get_test_cases, OcamlIntegerType};
use serde_bin_prot::{from_reader, to_writer, Nat0};

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
            | OcamlIntegerType::network64_int64 => Ok(Self::Int(i.1)),
            OcamlIntegerType::nat0 => {
                let v = u64::try_from(i.1).map_err(|_| "Tryed to parse signed int to nat0")?;
                Ok(Self::Nat0(Nat0::new(v)))
            }
            OcamlIntegerType::variant_int => Err("variant_int not supported (yet)".to_string()),
        }
    }
}

#[test]
fn test_serialize_integers() {
    for case_tuple in get_test_cases() {
        println!("{:?}", case_tuple);
        let (ocaml_type, bytes, integer) = case_tuple;

        let ocaml_integer =
            OCamlInteger::try_from((ocaml_type, integer)).expect("Invalid test case");

        // test serialization
        let mut output = Vec::<u8>::new();
        match ocaml_integer {
            OCamlInteger::Int(ref v) => {
                to_writer(&mut output, &v).unwrap();
            }
            OCamlInteger::Nat0(ref v) => {
                to_writer(&mut output, &v).unwrap();
            }
        }
        output.reverse(); // Not 100% sure why this is required. Seek confirmation.
        assert_eq!(output, bytes);
    }
}

#[test]
fn test_deserialize_integers() {
    for case_tuple in get_test_cases() {
        println!("{:?}", case_tuple);
        let (ocaml_type, mut bytes, integer) = case_tuple;

        let ocaml_integer =
            OCamlInteger::try_from((ocaml_type, integer)).expect("Invalid test case");

        // test deserialization
        match ocaml_integer {
            OCamlInteger::Int(_) => {
                bytes.reverse();
                let value: i64 = from_reader(bytes.as_slice()).expect("Deserialization failed");
                assert_eq!(value, integer)
            }
            OCamlInteger::Nat0(_) => {
                // TODO
            }
        }
    }
}
