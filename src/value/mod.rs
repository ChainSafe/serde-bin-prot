//! The Value enum, a loosely typed way of representing any valid bin_prot value.

mod de;
mod layout;

// use serde::{Serialize, Deserialize};

pub enum Value {
    Nat0(u32),
    Unit,
    Bool(bool),
    String(String),
    Char(char),
    Int(i64),
    Float(f64),
    Option(Box<Value>),
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

// pub fn to_value<T>(value: T) -> Result<Value, Error>
// where
//     T: Serialize,
// {
//     value.serialize(Serializer)
// }

// pub fn from_value<T>(value: Value) -> Result<T, Error>
// where
//     T: DeserializeOwned,
// {
//     T::deserialize(value)
// }
