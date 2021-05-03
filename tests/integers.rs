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
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_bin_prot::{to_writer, Nat0};

enum OCamlInteger {
    Int(i64),
    Nat0(Nat0),
}

impl  OCamlInteger {
    fn from_strs(type_s: &str, value_s: &str) -> Result<Self, String> {
        match type_s {
            "int"
            | "int32"
            | "int64"
            | "int_16bit"
            | "int_32bit"
            | "int_64bit"
            | "int64_bits"
            | "network32_int"
            | "network64_int"
            | "network32_int32"
            | "network64_int64" => {
                let v = i64::from_str_radix(value_s, 10).unwrap();
                Ok(Self::Int(v))
            },
            "nat0" => {
                let v = u64::from_str_radix(value_s, 10).unwrap();
                Ok(Self::Nat0(Nat0::new(v)))
            },
            "variant_int" => Err("Not supported".to_string()),
            _ => Err("fail".to_string())
        }
    }
}

#[derive(Debug)]
struct IntegerTestCase {
    pub bytes: Vec<u8>,
    pub number_string: String,
    pub type_string: String,
}

impl IntegerTestCase {
    /// Test that serializing the integer as the given type 
    /// yields the correct bytes
    pub fn test_ser(&self) {
        let val = OCamlInteger::from_strs(&self.type_string, &self.number_string).unwrap();
        let mut output = Vec::<u8>::new();
        match val {
            OCamlInteger::Int(v) => { to_writer(&mut output, &v).unwrap(); }
            OCamlInteger::Nat0(v) => { to_writer(&mut output, &v).unwrap(); }
        }
        assert_eq!(output, self.bytes)
    }
}

/// Reads a line of the OCaml test code and converts it into our test case struct
fn parse_line(s: &str) -> Result<IntegerTestCase, regex::Error> {
    // first capture is type (e.g. nat0, uint32 etc)
    // captures 2-10 are the bytes which are either hex bytes (e.g. ff) or '..' which is a placeholder
    // last capture is the base10 integer representation
    let r = Regex::new(
        r"^(\w+)\| (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) (\.{2}|[0-9a-fA-F]{2}) -> (-?\d+)$",
    )?;
    if let Some(caps) = r.captures(s) {
        let type_string = caps.get(1).unwrap().as_str().to_string();
        let mut bytes = Vec::<u8>::new();
        for i in 2..=10 {
            let s = caps.get(i).unwrap().as_str();
            if let Ok(byte) = u8::from_str_radix(s, 16) {
                bytes.push(byte)
            }
        }
        bytes.reverse();
        let number_string = caps.get(11).unwrap().as_str().to_string();

        Ok(IntegerTestCase {
            bytes,
            number_string,
            type_string,
        })
    } else {
        panic!()
    }
}

#[test]
fn test_parsing() {
    let filename = "tests/integers_repr_tests_64bit.ml";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().skip(6925).enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let result = parse_line(&line).unwrap();
        println!("{}, {:?}", index+1, line);
        result.test_ser();
    }
    assert!(true)
}
