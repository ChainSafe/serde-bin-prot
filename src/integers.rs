use crate::consts::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num::{FromPrimitive, Unsigned};
///
/// Bin_Prod uses a variable length encoding for integers based on their size.
///
/// For all Integer types:
/// if the value is positive (including zero) and if it is:
/// <  0x00000080  ->  lower 8 bits of the integer                     (1 byte)
/// <  0x00008000  ->  CODE_INT16 followed by lower 16 bits of integer (3 bytes)
/// <  0x80000000  ->  CODE_INT32 followed by lower 32 bits of integer (5 bytes)
/// >= 0x80000000  ->  CODE_INT64 followed by all 64 bits of integer   (9 bytes)
///
/// If the value is negative and if it is:
/// >= -0x00000080  ->  CODE_NEG_INT8 followed by lower 8 bits of integer (2 bytes)
/// >= -0x00008000  ->  CODE_INT16 followed by lower 16 bits of integer   (3 bytes)
/// >= -0x80000000  ->  CODE_INT32 followed by lower 32 bits of integer   (5 bytes)
/// <  -0x80000000  ->  CODE_INT64 followed by all 64 bits of integer     (9 bytes)
///
/// The little-endian format is used in the protocol for the contents of integers on all platforms
///
use std::io;

// extension trait for writers implementing io::Write to allow them to write any integer
// in bin_prot form.
// This accepts any integer which can be converted to an i64 (which is all integers in Rust)
pub trait WriteBinProtIntegerExt: io::Write {
    fn write_binprot_integer<T: Into<i64>>(&mut self, n: T) -> Result<(), io::Error> {
        let n: i64 = n.into();
        if n >= 0 {
            // positive or zero case
            match n {
                _ if n < 0x00000080 => self.write_u8(n as u8),
                _ if n < 0x00008000 => {
                    self.write_u8(CODE_INT16)?;
                    self.write_u16::<LittleEndian>(n as u16)
                }
                _ if n < 0x80000000 => {
                    self.write_u8(CODE_INT32)?;
                    self.write_u32::<LittleEndian>(n as u32)
                }
                _ => {
                    self.write_u8(CODE_INT64)?;
                    self.write_u64::<LittleEndian>(n as u64)
                }
            }
        } else {
            // negative case
            match n {
                _ if n >= -0x00000080 => {
                    self.write_u8(CODE_NEG_INT8)?;
                    self.write_i8(n as i8)
                }
                _ if n >= -0x00008000 => {
                    self.write_u8(CODE_INT16)?;
                    self.write_i16::<LittleEndian>(n as i16)
                }
                _ if n >= -0x80000000 => {
                    self.write_u8(CODE_INT32)?;
                    self.write_i32::<LittleEndian>(n as i32)
                }
                _ => {
                    self.write_u8(CODE_INT64)?;
                    self.write_i64::<LittleEndian>(n as i64)
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
    fn write_binprot_nat0<T: Into<u64>>(&mut self, n: T) -> Result<(), io::Error> {
        let n: u64 = n.into();
        match n {
            _ if n < 0x000000080 => self.write_u8(n as u8),
            _ if n < 0x000010000 => {
                self.write_u8(CODE_INT16)?;
                self.write_u16::<LittleEndian>(n as u16)
            }
            _ if n < 0x100000000 => {
                self.write_u8(CODE_INT32)?;
                self.write_u32::<LittleEndian>(n as u32)
            }
            _ => {
                self.write_u8(CODE_INT64)?;
                self.write_u64::<LittleEndian>(n as u64)
            }
        }
    }
}

/// All types that implement `Write` get methods defined in `WriteBinProtIntegerExt`
/// for free.
impl<W: io::Write + ?Sized> WriteBinProtIntegerExt for W {}

// Extension trait for readers implementing io::Read to allow them to read a bin_prot encoded
// integer
pub trait ReadBinProtIntegerExt: io::Read {
    fn read_binprot_integer<T: FromPrimitive>(&mut self) -> Result<T, io::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        // for the possibly signed cases, read them as signed and allow
        // the conversion to fail if trying to convert a negative value
        // to an unsigned integer
        match buf[0] {
            CODE_INT16 => { // positive or negative 16 bit int
            	T::from_i16(self.read_i16::<LittleEndian>()?)
            }
            CODE_INT32 => { // positive or negative 32 bit int
            	T::from_i32(self.read_i32::<LittleEndian>()?)
            }
            CODE_INT64 => { // positive or negative 64 bit int
            	T::from_i64(self.read_i64::<LittleEndian>()?)
            }
            CODE_NEG_INT8 => { // a negative signed i8
            	T::from_i8(self.read_i8()?)
            }
            byte0 => {
                // first byte isnt a code so interpret it as a u8
                assert!(byte0 < 0x000000080, "Invalid value stored in byte"); // sanity check
                T::from_u8(byte0)
            }
        }
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Destination integer type too small for value or incorrect sign",
        ))
    }

    fn read_binprot_nat0<T: FromPrimitive + Unsigned>(&mut self) -> Result<T, io::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        // In this case it is always reading an unsigned integer
        match buf[0] {
            CODE_INT16 => { // positive or negative 16 bit int
            	T::from_u16(self.read_u16::<LittleEndian>()?)
            }
            CODE_INT32 => { // positive or negative 32 bit int
            	T::from_u32(self.read_u32::<LittleEndian>()?)
            }
            CODE_INT64 => { // positive or negative 64 bit int
            	T::from_u64(self.read_u64::<LittleEndian>()?)
            }
            byte0 => {
                // first byte isnt a code so interpret it as a u8
                assert!(byte0 < 0x000000080, "Invalid value stored in byte"); // sanity check
                T::from_u8(byte0)
            }
        }
        .ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Destination integer type too small for value or incorrect sign",
        ))
    }
}

/// All types that implement `Read` get methods defined in `ReadBinProtIntegerExt`
/// for free.
impl<W: io::Read + ?Sized> ReadBinProtIntegerExt for W {}
