use difference::Changeset;
use serde_bin_prot::to_writer;
use std::f64;
use std::fmt::Write;

mod common;
use common::print_byte_array;

#[test]
fn test_float() {
    bin_prot_test! {
        0x3c 0xb0 00 00 00 00 00 00 -> 2.2204460492503131E-16,
        0x7f 0xf0 00 00 00 00 00 00 -> f64::INFINITY,
        0x7f 0xef 0xff 0xff 0xff 0xff 0xff 0xff -> 1.7976931348623157E+308,
        0x7f 0xf0 00 00 00 00 00 00 -> f64::INFINITY,
        00 0x10 00 00 00 00 00 00 -> 2.2250738585072014E-308,
        00 00 00 00 00 00 00 0x01 -> 4.94065645841247E-324,
        0xff 0xf0 00 00 00 00 00 00 -> f64::NEG_INFINITY,
        0xbf 0xf0 00 00 00 00 00 00 -> -1f64,
        0xff 0xf0 00 00 00 00 00 00 -> f64::NEG_INFINITY,
        0x3f 0xf0 00 00 00 00 00 00 -> 1f64,
        0x3e 0x7a 0xd7 0xf2 0x9a 0xbc 0xaf 0x48 -> 1E-07
        // 00 00 00 00 00 00 00 00 -> 0 // FIXME
    }
}
