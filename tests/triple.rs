use serde_bin_prot::to_writer;

mod common;
use common::TestCase;

#[test]
fn test_triple() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. .. .. 0x00 0x00 0x00 -> ((0, 0, 0)),
        .. .. .. .. .. .. .. .. .. .. .. .. 0x01 0x01 0x01 -> ((1, 1, 1)),
        .. .. .. .. .. .. .. .. .. 0xff 0xff 0xff 0xff 0xff 0xff -> ((-1, -1, -1)),
        0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xfd 0x7f 0xff 0xff 0xff 0xfd -> ((2147483647, 2147483647, 2147483647)),
        0x80 0x00 0x00 0x00 0xfd 0x80 0x00 0x00 0x00 0xfd 0x80 0x00 0x00 0x00 0xfd -> ((-2147483648, -2147483648, -2147483648))
    }
}

fn triple_test_cases() -> Vec<TestCase<(i32, i32, i32)>> {
    vec![
        TestCase::new((0, 0, 0), vec![0x00; 3]),
        TestCase::new((1, 1, 1), vec![0x01; 3]),
        TestCase::new((-1, -1, -1), vec![0xff; 6]),
        TestCase::new(
            (2147483647, 2147483647, 2147483647),
            vec![
                0x7f, 0xff, 0xff, 0xff, 0xfd, 0x7f, 0xff, 0xff, 0xff, 0xfd, 0x7f, 0xff, 0xff, 0xff,
                0xfd,
            ],
        ),
        TestCase::new(
            (-2147483648, -2147483648, -2147483648),
            vec![
                0x80, 0x00, 0x00, 0x00, 0xfd, 0x80, 0x00, 0x00, 0x00, 0xfd, 0x80, 0x00, 0x00, 0x00,
                0xfd,
            ],
        ),
    ]
}

#[test]
fn test_serialize_triples() {
    for mut val in triple_test_cases() {
        let mut output = Vec::<u8>::new();
        val.expected.reverse();
        to_writer(&mut output, &val.input).unwrap();
        assert_eq!(val.expected, output);
    }
}

#[test]
fn test_roundtrip_triples() {
    for val in triple_test_cases() {
        common::roundtrip_test(val.input);
    }
}
