use serde_bin_prot::to_writer;

mod common;
use common::TestCase;

fn pair_test_cases() -> Vec<TestCase<(i32, i32)>> {
    vec![
        TestCase::new((0, 0), vec![0x00; 2]),
        TestCase::new((1, 1), vec![0x01; 2]),
        TestCase::new((-1, -1), vec![0xff; 4]),
        TestCase::new(
            (2147483647, 2147483647),
            vec![0x7f, 0xff, 0xff, 0xff, 0xfd, 0x7f, 0xff, 0xff, 0xff, 0xfd],
        ),
        TestCase::new(
            (-2147483648, -2147483648),
            vec![0x80, 0x00, 0x00, 0x00, 0xfd, 0x80, 0x00, 0x00, 0x00, 0xfd],
        ),
    ]
}

#[test]
fn test_serialize_pairs() {
    for mut val in pair_test_cases() {
        let mut output = Vec::<u8>::new();
        val.expected.reverse();
        to_writer(&mut output, &val.input).unwrap();
        assert_eq!(val.expected, output);
    }
}

#[test]
fn test_roundtrip_pairs() {
    for val in pair_test_cases() {
        common::roundtrip_test(val.input);
    }
}
