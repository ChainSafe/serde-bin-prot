use serde_bin_prot::to_writer;

mod common;
use common::TestCase;

fn option_test_cases() -> Vec<TestCase<Option<i64>>> {
    vec![
        TestCase::new(None, vec![0x00]),
        TestCase::new(Some(0), vec![0x00, 0x01]),
        TestCase::new(Some(1), vec![0x01, 0x01]),
        TestCase::new(Some(-1), vec![0xff, 0xff, 0x01]),
        TestCase::new(Some(2147483647), vec![0x7f, 0xff, 0xff, 0xff, 0xfd, 0x01]),
        //TestCase::new(Some(-2147483647), vec![0x80, 0x00, 0x00, 0x00, 0xfd, 0x01]),
    ]
}

#[test]
fn test_serialize_options() {
    for mut val in option_test_cases() {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val.input).unwrap();
        val.expected.reverse();
        assert_eq!(val.expected, output);
    }
}