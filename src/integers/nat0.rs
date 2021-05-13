use std::io::Cursor;
use crate::consts::*;
use byteorder::{LittleEndian, ReadBytesExt};
use num::{Unsigned};
use serde::ser::Serializer;
use serde::de::{self, Visitor, Deserializer};
use crate::write_ext::WriteBinProtExt;
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


pub fn serialize<T, S>(n: &T, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer, T: Unsigned + Into<u64> + Copy
{
    let mut bytes = Vec::<u8>::new();
    bytes.bin_write_nat0(*n).unwrap();
    s.serialize_bytes(&bytes)
}

struct Nat0Visitor;

impl<'de> Visitor<'de> for Nat0Visitor {
    type Value = usize;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "A bin_prot encoded integer (1, 3, 5, or 9 bytes depending on size)",
        )
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut rdr = Cursor::new(value);
        let byte = rdr.read_u8().unwrap();
        let result = match byte {
            CODE_INT16 => rdr.read_i16::<LittleEndian>()
                    .map_err(|_| de::Error::custom("could not read enough bytes for Nat0"))?
                    as usize,
            CODE_INT32 => rdr.read_i32::<LittleEndian>()
                    .map_err(|_| de::Error::custom("could not read enough bytes for Nat0"))?
                    as usize,
            CODE_INT64 => rdr.read_u64::<LittleEndian>()
                    .map_err(|_| de::Error::custom("could not read enough bytes for Nat0"))?
                    as usize,
            byte => byte as usize,
        };
        Ok(result)
    }
}

pub fn deserialize<'de, D>(d: D) -> Result<usize, D::Error>
    where D: Deserializer<'de>
{
    d.deserialize_bytes(Nat0Visitor)
}
