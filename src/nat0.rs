use crate::consts::*;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shrinkwraprs::Shrinkwrap;
use std::fmt;
use std::io::Cursor;

// This type encodes natural numbers including zero.
// It is frequently used by Bin_prot itself for encoding size information for e.g. lists, arrays, etc., and hence defined first here.
// Developers can reuse this type in their code, too, of course.
//
// If the value of the underlying integer is lower than a certain range, this implies a certain encoding as provided on the right hand side of the following definitions:
//
// <  0x000000080  ->  lower 8 bits of the integer                     (1 byte)
// <  0x000010000  ->  CODE_INT16 followed by lower 16 bits of integer (3 bytes)
// <  0x100000000  ->  CODE_INT32 followed by lower 32 bits of integer (5 bytes)
// >= 0x100000000  ->  CODE_INT64 followed by all 64 bits of integer   (9 bytes)

#[derive(Shrinkwrap, Debug)]
pub struct Nat0(u64);

impl Nat0 {
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    pub fn take(self) -> u64 {
        self.0
    }
}

impl From<usize> for Nat0 {
    fn from(v: usize) -> Self {
        Self(v as u64)
    }
}

impl Serialize for Nat0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = self.0;
        match v {
            _ if v < 0x000000080 => serializer.serialize_bytes(&v.to_le_bytes()[..1]),
            _ if v < 0x000010000 => {
                serializer.serialize_bytes(&[&[CODE_INT16], &v.to_le_bytes()[..2]].concat())
            }
            _ if v < 0x100000000 => {
                serializer.serialize_bytes(&[&[CODE_INT32], &v.to_le_bytes()[..4]].concat())
            }
            _ => serializer.serialize_bytes(&[&[CODE_INT64], &v.to_le_bytes()[..]].concat()),
        }
    }
}

struct Nat0Visitor;

impl<'de> Visitor<'de> for Nat0Visitor {
    type Value = Nat0;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "A bin_prot encoded Nat0 unsigned integer (1, 3, 5, or 9 bytes depending on size)",
        )
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut rdr = Cursor::new(value);
        let byte = rdr.read_u8().unwrap();
        let result = match byte {
            CODE_INT16 => Nat0(
                rdr.read_u16::<LittleEndian>()
                    .map_err(|_| de::Error::custom("could not read enough bytes for Nat0"))?
                    as u64,
            ),
            CODE_INT32 => Nat0(
                rdr.read_u32::<LittleEndian>()
                    .map_err(|_| de::Error::custom("could not read enough bytes for Nat0"))?
                    as u64,
            ),
            CODE_INT64 => Nat0(
                rdr.read_u64::<LittleEndian>()
                    .map_err(|_| de::Error::custom("could not read enough bytes for Nat0"))?
                    as u64,
            ),
            byte => {
                // it is a unsigned value < 0x80 so bytes so no prefix
                if byte < 0x000000080 {
                    Nat0(byte as u64)
                } else {
                    return Err(de::Error::custom("First byte of Nat0 did not match an int size code but byte value too large (not < 0x80)"));
                }
            }
        };
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for Nat0 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(Nat0Visitor)
    }
}
