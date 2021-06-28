use serde_bin_prot::to_writer;

mod common;
use common::TestCase;

fn bool_test_cases() -> Vec<TestCase<bool>> {
    vec![TestCase::new(true, vec![1]), TestCase::new(false, vec![0])]
}

#[test]
fn test_serialize_bools() {
    for val in bool_test_cases() {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val.input).unwrap();
        assert_eq!(val.expected, output);
    }
}

#[test]
fn test_roundtrip_bools() {
    for val in bool_test_cases() {
        common::roundtrip_test(val.input);
    }
}
