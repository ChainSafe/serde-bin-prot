///
/// These tests aim to follow the Jane Street OCaml tests as closely as possible
/// The tests contain a lookup table for auto-generated integer encoding test cases
///
/// Note the bytes are little-endian encoded.
///
/// These tests can be parsed and executed directly from their source

/// We can't generate reference output for every possible values as it would be huge,
/// instead we choose a few interesting points and generate tests in a window around
/// them. The points we choose are:

/// - 0
/// - min value
/// - max value
/// - all powers of 2 between min and max
/// - all points where the length of the serialized output change

/// For the last item, instead of hard-coding these points for every function, we find
/// them. This is to avoid errors. To find them we make the assumption that functions using
/// a variable length encoding respect the following:

/// - the encoded size is decreasing from min value to 0
/// - the encoded size is increasing from 0 to max value

/// Which is the basic assumption make by bin_prot: integers close to 0 are more frequent
/// and should occupy less space.

use std::collections::HashSet;
use std::io::Write;
mod common;

type Points = HashSet<i64>;

const TEST_WINDOW_LEN: i64 = 16;

const INT_MIN: i64 = -2_i64.pow(62);
const INT_MAX: i64 = 2_i64.pow(62) - 1;

struct TestCase {
    min: i64,
    max: i64,
}

fn find_interesting_points(case: TestCase) -> Points {
    let mut points = HashSet::new();
    points.insert(0);
    points.insert(case.min);
    points.insert(case.max);
    points = points.union(&valid_powers_of_two(&case)).cloned().collect();
    add_windows_around_points(case, points)
}

// { 2 ^ n | 0 <= n <= 63 } \/ { -(2 ^ n) | 0 <= n <= 63 }
fn powers_of_two() -> Points {
    let mut acc = Points::new();
    for i in 0..62 {
        acc.insert(2_i64.pow(i));
        acc.insert(-2_i64.pow(i));
    }
    acc
}

fn valid_powers_of_two(case: &TestCase) -> Points {
    powers_of_two()
        .into_iter()
        .filter(|i| i >= &case.min && i <= &case.max)
        .collect()
}

fn add_windows_around_points(TestCase { min, max }: TestCase, points: Points) -> Points {
    // add all point between a and b inclusive into an accumulator
    fn add_between(a: i64, b: i64, mut acc: Points) -> Points {
        println!("adding between {} and {}", a, b);
        for i in a..b {
            acc.insert(i.clone());
        }
        acc
    }

    points.into_iter().fold(Points::new(), |acc, i| {
        let d = TEST_WINDOW_LEN / 2;
        let a = if i <= (min + d) { min } else { i - d };
        let b = if i >= (max - d) { max } else { i + d };
        add_between(a, b, acc)
    })
}

/// Test the variable size integer encoding
/// Additionally, it outputs the byte array to integer value mapping like: https://github.com/janestreet/bin_prot/blob/5915cde59105f398b53f682c5f4dad29e272f696/test/integers_repr_tests_64bit.ml
#[test]
fn test_roundtrip_integers() {
    let int_test = TestCase {
        min: INT_MIN,
        max: INT_MAX,
    };

    let mut f = std::fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open("tests/integers_repr.bin")
        .unwrap();
    f.write(
        "// WARNING: never accept the corrected output for this file, it must never change!\n\n\n"
            .as_bytes(),
    )
    .unwrap();

    for val in find_interesting_points(int_test) {
        common::write_integer_test_repr(val, 9, &mut f); // 9: 8 + extra `fd` byte marker for bin prot
        common::roundtrip_test(val);
    }
}
