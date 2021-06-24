//use difference::Changeset;
use serde_bin_prot::to_writer;
use std::fmt::Write;

mod common;

#[derive(Debug)]
pub struct TestCase<T> {
    input: T,
    expected: Vec<u8>,
}

impl<T> TestCase<T> {
    pub fn new(input: T, expected: Vec<u8>) -> Self {
        TestCase {
            input: input,
            expected: expected,
        }
    }
}

fn bool_test_cases() -> Vec<TestCase<bool>> {
    vec![TestCase::new(true, vec![1]), TestCase::new(false, vec![0])]
}

#[test]
fn test_serialize_bools() {
    let mut buf = String::new();
    writeln!(&mut buf).unwrap();

    for val in bool_test_cases() {
        //println!("case {:?}", val);
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val.input).unwrap();
        //println!("out {:?}", output);
        assert_eq!(val.expected, output);
    }
}

#[test]
fn test_roundtrip_bools() {
    for val in bool_test_cases() {
        common::roundtrip_test(val.input);
    }
}
