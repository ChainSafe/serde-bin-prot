//! The Value enum, a loosely typed way of representing any valid bin_prot value.

use serde::Deserialize;

mod visitor;
pub mod layout;

use visitor::ValueVisitor;

#[derive(Clone, Debug, PartialEq)]
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
}

impl Default for Value {
    fn default() -> Value {
        Value::Unit
    }
}

// Ensure the value visitor is always used when deserializing to a Value
// This will always request deserialize_any be called since the Value implementation
// does not describe its own structure. Attempting to deserialize into Value from a
// non-self describing format will result in an error
impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}
