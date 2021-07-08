use serde_bin_prot::to_writer;

mod common;
use common::TestCase;

fn char_test_cases() -> Vec<TestCase<char>> {
    vec![
        TestCase::new('\u{000}', vec![0x00]),
        TestCase::new('A', vec![0x41]),
        TestCase::new('z', vec![0x7a]),
        TestCase::new(';', vec![0x3b]),
        // TestCase::new('\u{255}', vec![0xff]),
    ]
}

#[test]
fn test_serialize_chars() {
    for val in char_test_cases() {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val.input).unwrap();
        assert_eq!(val.expected, output);
    }
}
