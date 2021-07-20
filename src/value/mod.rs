//! The Value enum, a loosely typed way of representing any valid bin_prot value.

use crate::value::de::ValueVisitor;
use serde::Deserialize;

mod de;
mod layout;

// use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub enum Value {
    Unit,
    Nat0(u32),
    Bool(bool),
    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Option(Option<Box<Value>>),
    Record(Vec<(String, Value)>), // records/structs
    Tuple(Vec<Value>),
    Sum {
        name: String,
        index: usize,
        value: Box<Value>,
    }, // sum types/enums
    List(Vec<Value>),

    Placeholder, // This is a special variant to be used when iteratively building up the type from data
}

impl Default for Value {
    fn default() -> Value {
        Value::Unit
    }
}

// ensure the value visitor is always used when deserializing to a Value
impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
