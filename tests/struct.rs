use serde::{Deserialize, Serialize};
use serde_bin_prot::to_writer;
use std::f64::INFINITY;

mod common;

#[derive(Debug)]
enum TestCaseType {
    TestA(A),
    TestB(B),
}

#[derive(Debug)]
struct TestCase {
    input: TestCaseType,
    expected: Vec<u8>,
}

impl TestCase {
    pub fn new(input: TestCaseType, expected: Vec<u8>) -> Self {
        TestCase {
            input: input,
            expected: expected,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct A {
    x: i64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct b_inner {
    w: i64,
    x: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct B {
    y: b_inner,
    z: Option<i64>,
}

fn struct_test_cases() -> Vec<TestCase> {
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
        TestCase::new(TestCaseType::TestA(a0), a0_expected),
        TestCase::new(TestCaseType::TestA(a1), a1_expected),
        TestCase::new(TestCaseType::TestB(b0), b0_expected),
        TestCase::new(TestCaseType::TestB(b1), b1_expected),
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
            TestCaseType::TestA(input) => encode_and_compare(input, val.expected),
            TestCaseType::TestB(input) => encode_and_compare(input, val.expected),
        };
    }
}
