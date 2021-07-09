
mod common;

#[test]
fn test_roundtrip_arrays() {
    let test_case:[i32;0] = [];
    common::roundtrip_test(test_case);
    let test_case = [0i32];
    common::roundtrip_test(test_case);
    let test_case = [0i32, 1];
    common::roundtrip_test(test_case);
    let test_case = [1i32, -1];
    common::roundtrip_test(test_case);
    let test_case = [0i32, 2147483647];
    common::roundtrip_test(test_case);
    let test_case = [2147483647i32, -2147483648];
    common::roundtrip_test(test_case);
}
