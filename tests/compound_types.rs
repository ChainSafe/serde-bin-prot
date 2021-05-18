use serde::{Deserialize, Serialize};
use serde_bin_prot::integers::{integer, nat0};
mod common;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestTupleStruct(bool, i8, i16, i32, i64, (), Option<()>, [u8; 3], char);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct A(bool);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct B {
    a: A,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestFieldAttrs {
    #[serde(with = "nat0")]
    n: u8,
    #[serde(with = "integer")]
    i: i16,
}

#[test]
fn roundtrip_tuple_struct() {
    common::roundtrip_test(TestTupleStruct(
        true,
        0,
        0,
        0,
        0,
        (),
        None,
        [0x01, 0x02, 0x03],
        'c',
    ));
}

#[test]
fn roundtrip_nested_structs() {
    common::roundtrip_test(B { a: A(false) });
}
