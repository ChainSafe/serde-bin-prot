use serde::{Deserialize, Serialize};
use serde_bin_prot::to_writer;
use std::f64::INFINITY;

mod common;
use common::TestCase;

#[derive(Debug)]
enum StructTestCases {
    TestA(A),
    TestB(B),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct A {
    x: i64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct B {
    y: b_inner,
    z: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct b_inner {
    w: i64,
    x: i64,
}

fn struct_test_cases() -> Vec<TestCase<StructTestCases>> {
    let a0 = A { x: 0, y: 0.0 };

    let a0_expected = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    let a1 = A {
        x: 2147483647,
        y: INFINITY,
    };

    let mut a1_expected = vec![
        0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff, 0xfd,
    ];
    a1_expected.reverse();

    let b0 = B {
        y: b_inner { w: 0, x: 0 },
        z: None,
    };

    let b0_expected = vec![0, 0, 0];

    let b1 = B {
        y: b_inner {
            w: 9223372036854775807,
            x: 2147483647,
        },
        z: None,
    };

    let mut b1_expected = vec![
        0x00, 0x7f, 0xff, 0xff, 0xff, 0xfd, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc,
    ];
    b1_expected.reverse();

    vec![
        TestCase::new(StructTestCases::TestA(a0), a0_expected),
        TestCase::new(StructTestCases::TestA(a1), a1_expected),
        TestCase::new(StructTestCases::TestB(b0), b0_expected),
        TestCase::new(StructTestCases::TestB(b1), b1_expected),
    ]
}

#[test]
fn test_serialize_structs() {
    for val in struct_test_cases() {
        fn encode_and_compare<T: Serialize>(input: T, expected: Vec<u8>) {
            let mut output = Vec::<u8>::new();
            to_writer(&mut output, &input).unwrap();
            assert_eq!(expected, output);
        }

        match val.input {
            StructTestCases::TestA(input) => encode_and_compare(input, val.expected),
            StructTestCases::TestB(input) => encode_and_compare(input, val.expected),
        };
    }
}

#[test]
fn test_roundtrip_structs() {
    for val in struct_test_cases() {
        match val.input {
            StructTestCases::TestA(input) => common::roundtrip_test(input),
            StructTestCases::TestB(input) => common::roundtrip_test(input),
        };
    }
}
