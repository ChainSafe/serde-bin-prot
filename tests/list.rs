
// list in bin prot is equivalent to a Vec

mod common;

#[test]
fn test_list() {
    bin_prot_test! {
        .. .. .. .. .. .. .. .. .. .. 0x00 -> (Vec::<i32>::new()),
        .. .. .. .. .. .. .. .. .. 0x00 0x01 -> (vec![0i32;1]),
        .. .. .. .. .. .. .. .. 0x01 0x00 0x02 -> (vec![0i32, 1]),
        .. .. .. .. .. .. .. 0xff 0xff 0x01 0x02 -> (vec![1i32, -1]),
        .. .. .. .. 0x7f 0xff 0xff 0xff 0xfd 0x00 0x02 -> (vec![0i32, 2147483647]),
        0x80 0x00 0x00 0x00 0xfd 0x7f 0xff 0xff 0xff 0xfd 0x02 -> (vec![2147483647i32, -2147483648])
    }
}
