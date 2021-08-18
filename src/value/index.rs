use super::Value;
use std::ops;

/// A type that can be used to index into a `bin_prot::Value`.
pub trait Index {
    /// Return None if the key is not already in the array or object.
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
}

// Numeric indexing, only compatible with List and Tuple (or sum types containing either of these)
impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::List(ref vec) | Value::Tuple(ref vec) => vec.get(*self),
            Value::Sum { ref value, .. } => match **value {
                Value::List(ref vec) | Value::Tuple(ref vec) => vec.get(*self),
                _ => None,
            },
            _ => None,
        }
    }
}

// String indexing. Only compatible with Record (or sum types containg a record)
impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match *v {
            Value::Record(ref map) => map.get(self),
            Value::Sum { ref value, .. } => match **value {
                Value::Record(ref map) => map.get(self),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
}

impl<'a, T> Index for &'a T
where
    T: ?Sized + Index,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        (**self).index_into(v)
    }
}

impl<I> ops::Index<I> for Value
where
    I: Index + std::fmt::Display,
{
    type Output = Value;

    // The ops::Index will panic when indexing a value that doesn't exist
    // This is consistent to the behaviour when indexing into an array
    fn index(&self, index: I) -> &Value {
        index
            .index_into(self)
            .unwrap_or_else(|| panic!("No value for index: {}", index))
    }
}
