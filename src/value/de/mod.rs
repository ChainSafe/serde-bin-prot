use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use std::io::BufReader;
use std::io::Read;

use crate::error::{Error, Result};
use crate::value::layout::BinProtRule;
use crate::value::Value;

mod visitor;

use visitor::ValueVisitor;

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

pub struct Deserializer<R: Read> {
    rdr: BufReader<R>,
    layout: BinProtRule,
}

impl<R: Read> Deserializer<R> {
    fn new(rdr: R, layout: BinProtRule) -> Self {
        Self {
            rdr: BufReader::new(rdr),
            layout,
        }
    }
}

// impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
//     type Error = Error;
//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
//     where
//         V: Visitor<'de>,
//     {
//         // Rather than relying on the visitor hints, use the layout
//         // to determine what type to read next
//     }

//     serde::forward_to_deserialize_any! {
//         bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
//         bytes byte_buf option unit unit_struct newtype_struct seq tuple
//         tuple_struct map struct enum identifier ignored_any
//     }
// }
