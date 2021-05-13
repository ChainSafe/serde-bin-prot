use serde_bin_prot::{from_reader, to_writer};

fn get_test_cases() -> Vec<(Vec<u8>, Vec<i64>)> {
    vec![
        (vec![0x00], vec![]),
        (vec![0x00, 0x01], vec![0]),
        (vec![0x01, 0x00, 0x02], vec![0, 1]),
        (vec![0xff, 0xff, 0x01, 0x02], vec![1, -1]),
        (
            vec![0x7f, 0xff, 0xff, 0xff, 0xfd, 0x00, 0x02],
            vec![0, 2147483647],
        ),
        (
            vec![
                0x80, 0x00, 0x00, 0x00, 0xfd, 0x7f, 0xff, 0xff, 0xff, 0xfd, 0x02,
            ],
            vec![2147483647, -2147483648],
        ),
    ]
}

#[test]
fn test_serialize_arrays() {
    for (expected_bytes, val) in get_test_cases() {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val).unwrap();
        output.reverse();
        assert_eq!(output, expected_bytes);
    }
}

#[test]
fn test_deserialize_arrays() {
    for (mut bytes, expected_val) in get_test_cases() {
        bytes.reverse();

        let value: Vec<i64> = from_reader(bytes.as_slice()).expect("Deserialization failed");
        assert_eq!(value, expected_val);
    }
}
