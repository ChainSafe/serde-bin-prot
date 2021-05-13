use std::fmt::Write;

/// Prints a byte array according to the style used in the Jane Street
/// bin_prot tests. Byte array is reversed and padded up to max length with ..
/// Bytes are written in hex with lowercase letters and no 0x prefix
pub fn print_byte_array<W: Write>(w: &mut W, bytes: &[u8], max_len: usize) {
    let padding = max_len - bytes.len();
    for _ in 0..padding {
        write!(w, ".. ").unwrap();
    }

    for b in bytes.iter().rev() {
        write!(w, "{:02x} ", b).unwrap();
    }
}
