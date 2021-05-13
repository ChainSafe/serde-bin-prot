use std::io::Cursor;

use crate::{WriteBinProtExt, ReadBinProtExt};
use serde::ser::Serializer;
use serde::de::{self, Visitor, Deserializer};


pub fn serialize<T, S>(n: &T, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer, T: Into<i64> + Copy
{
    let mut bytes = Vec::<u8>::new();
    bytes.bin_write_integer(*n).unwrap();
    s.serialize_bytes(&bytes)
}

struct IntegerVisitor;

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = i64;

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
        rdr.bin_read_integer().map_err(|_| de::Error::custom(""))
    }
}

pub fn deserialize<'de, D>(d: D) -> Result<i64, D::Error>
    where D: Deserializer<'de>
{
    d.deserialize_bytes(IntegerVisitor)
}
