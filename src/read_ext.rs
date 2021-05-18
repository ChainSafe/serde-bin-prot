use crate::consts::*;
use byteorder::{LittleEndian, ReadBytesExt};
use num::{FromPrimitive, Unsigned};
use std::io;

// Extension trait for readers implementing io::Read to allow them to read a bin_prot encoded
// integer
pub trait ReadBinProtExt: io::Read {
    fn bin_read_unit(&mut self) -> Result<(), io::Error> {
        match self.read_u8()? {
            0x00 => Ok(()),
            b => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid unit byte. Expected 0x00, found {:}", b),
            )),
        }
    }

    fn bin_read_bool(&mut self) -> Result<bool, io::Error> {
        match self.read_u8()? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            b => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Invalid boolean byte. Expected either 0x00 or 0x01, found {:}",
                    b
                ),
            )),
        }
    }

    // This function individual bytes from the reader and appends them to a buffer
    // With each new byte it attempts to convert the buffer to a utf-8 char and
    // failing this will continue until the max number of bytes for a char
    // is encountered
    fn bin_read_char(&mut self) -> Result<char, io::Error> {
        let mut buf = [0; 4];
        for i in 0..4 {
            buf[i] = self.read_u8()?;
            if let Ok(s) = core::str::from_utf8(&buf[..=i]) {
                // can unwrap here as if from_utf8 returned Ok
                // then there is at least one char in the string
                return Ok(s.chars().next().unwrap());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not construct valid UTF-8 char from bytes",
        ))
    }

    fn bin_read_integer<T: FromPrimitive>(&mut self) -> Result<T, io::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        // for the possibly signed cases, read them as signed and allow
        // the conversion to fail if trying to convert a negative value
        // to an unsigned integer
        match buf[0] {
            CODE_INT16 => {
                // positive or negative 16 bit int
                T::from_i16(self.read_i16::<LittleEndian>()?)
            }
            CODE_INT32 => {
                // positive or negative 32 bit int
                T::from_i32(self.read_i32::<LittleEndian>()?)
            }
            CODE_INT64 => {
                // positive or negative 64 bit int
                T::from_i64(self.read_i64::<LittleEndian>()?)
            }
            CODE_NEG_INT8 => {
                // a negative signed i8
                T::from_i8(self.read_i8()?)
            }
            byte0 => {
                // first byte isnt a code so interpret it as a u8
                assert!(byte0 < 0x000000080, "Invalid value stored in byte"); // sanity check
                T::from_u8(byte0)
            }
        }
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Destination integer type too small for value or incorrect sign",
        ))
    }

    fn bin_read_nat0<T: FromPrimitive + Unsigned>(&mut self) -> Result<T, io::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        // In this case it is always reading an unsigned integer
        match buf[0] {
            CODE_INT16 => {
                // positive or negative 16 bit int
                T::from_u16(self.read_u16::<LittleEndian>()?)
            }
            CODE_INT32 => {
                // positive or negative 32 bit int
                T::from_u32(self.read_u32::<LittleEndian>()?)
            }
            CODE_INT64 => {
                // positive or negative 64 bit int
                T::from_u64(self.read_u64::<LittleEndian>()?)
            }
            byte0 => {
                // first byte isnt a code so interpret it as a u8
                assert!(byte0 < 0x000000080, "Invalid value stored in byte"); // sanity check
                T::from_u8(byte0)
            }
        }
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData,
            "Destination integer type too small for value or incorrect sign",
        ))
    }
}

/// All types that implement `Read` get methods defined in `ReadBinProtIntegerExt`
/// for free.
impl<W: io::Read + ?Sized> ReadBinProtExt for W {}
