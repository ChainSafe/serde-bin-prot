use difference::Changeset;
use serde_bin_prot::{to_writer};
use std::f64;
use std::fmt::Write;

mod common;
use common::print_byte_array;

const MAX_BYTES: usize = 8;

const EXPECTED: &str = r#"
7f f0 00 00 00 00 00 00 -> inf
ff f0 00 00 00 00 00 00 -> -inf
3c b0 00 00 00 00 00 00 -> 2.2204460492503131E-16
7f ef ff ff ff ff ff ff -> 1.7976931348623157E308
bf f0 00 00 00 00 00 00 -> -1.0000000000000000E0
3f f0 00 00 00 00 00 00 -> 1.0000000000000000E0
00 00 00 00 00 00 00 00 -> 0.0000000000000000E0
"#;

const TEST_CASES: &[f64] = &[
    f64::INFINITY,
    f64::NEG_INFINITY, // min positive subnormal
    f64::EPSILON,
    f64::MAX, // max finnite
    -1.0,
    1.0,
    0.0,
];

#[test]
fn test_serialize_floats() {
    let mut buf = String::new();
    writeln!(&mut buf).unwrap();
    for val in TEST_CASES {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val).unwrap();
        print_byte_array(&mut buf, &output, MAX_BYTES);
        writeln!(&mut buf, "-> {:.16E}", val).expect("its cooked");
    }

    let cs = Changeset::new(&buf, EXPECTED, "");
    println!("{}", cs);
    assert_eq!(cs.distance, 0)
}

#[test]
fn test_roundtrip_floats() {
    for val in TEST_CASES {
        common::roundtrip_test(*val);
    }
}
