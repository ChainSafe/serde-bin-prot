use crate::consts::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num::{FromPrimitive, Unsigned};

/// Extension traits for io::Read and io::Write to read and
/// write bin_prot encoded types
use std::io;

// extension trait for writers implementing io::Write to allow them to write any integer
// in bin_prot form.
// This accepts any integer which can be converted to an i64 (which is all integers in Rust)
pub trait WriteBinProtExt: io::Write {
    fn bin_write_integer<T: Into<i64>>(&mut self, n: T) -> Result<usize, io::Error> {
        let n: i64 = n.into();
        if n >= 0 {
            // positive or zero case
            match n {
                _ if n < 0x00000080 => self.write_u8(n as u8).map(|_| 1),
                _ if n < 0x00008000 => {
                    self.write_u8(CODE_INT16)?;
                    self.write_u16::<LittleEndian>(n as u16).map(|_| 3)
                }
                _ if n < 0x80000000 => {
                    self.write_u8(CODE_INT32)?;
                    self.write_u32::<LittleEndian>(n as u32).map(|_| 5)
                }
                _ => {
                    self.write_u8(CODE_INT64)?;
                    self.write_u64::<LittleEndian>(n as u64).map(|_| 9)
                }
            }
        } else {
            // negative case
            match n {
                _ if n >= -0x00000080 => {
                    self.write_u8(CODE_NEG_INT8)?;
                    self.write_i8(n as i8).map(|_| 2)
                }
                _ if n >= -0x00008000 => {
                    self.write_u8(CODE_INT16)?;
                    self.write_i16::<LittleEndian>(n as i16).map(|_| 3)
                }
                _ if n >= -0x80000000 => {
                    self.write_u8(CODE_INT32)?;
                    self.write_i32::<LittleEndian>(n as i32).map(|_| 5)
                }
                _ => {
                    self.write_u8(CODE_INT64)?;
                    self.write_i64::<LittleEndian>(n as i64).map(|_| 9)
                }
            }
        }
    }

    // bin_prot also supports a slightly different encoding called Nat0
    // This is an unsigned integer type that is used internally by the protocol
    // for storing sizes of lists etc.
    // <  0x000000080  ->  lower 8 bits of the integer                     (1 byte)
    // <  0x000010000  ->  CODE_INT16 followed by lower 16 bits of integer (3 bytes)
    // <  0x100000000  ->  CODE_INT32 followed by lower 32 bits of integer (5 bytes)
    // >= 0x100000000  ->  CODE_INT64 followed by all 64 bits of integer   (9 bytes)
    fn bin_write_nat0<T: Into<u64>>(&mut self, n: T) -> Result<usize, io::Error> {
        let n: u64 = n.into();
        match n {
            _ if n < 0x000000080 => self.write_u8(n as u8).map(|_| 1),
            _ if n < 0x000010000 => {
                self.write_u8(CODE_INT16)?;
                self.write_u16::<LittleEndian>(n as u16).map(|_| 3)
            }
            _ if n < 0x100000000 => {
                self.write_u8(CODE_INT32)?;
                self.write_u32::<LittleEndian>(n as u32).map(|_| 5)
            }
            _ => {
                self.write_u8(CODE_INT64)?;
                self.write_u64::<LittleEndian>(n as u64).map(|_| 9)
            }
        }
    }
}

/// All types that implement `Write` get methods defined in `WriteBinProtIntegerExt`
/// for free.
impl<W: io::Write + ?Sized> WriteBinProtExt for W {}
