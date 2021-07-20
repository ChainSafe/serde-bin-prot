use crate::value::Value;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;

pub struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any valid JSON value")
    }

    #[inline]
    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Int(value.into()))
    }

    // #[inline]
    // fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
    //     Ok(Value::Int(value.into()))
    // }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::Float(value))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(String::from(value))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Unit)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Unit)
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(Value::List(vec))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        match visitor.next_key_seed(KeyClassifier)? {
            #[cfg(feature = "arbitrary_precision")]
            Some(KeyClass::Number) => {
                let number: NumberFromString = visitor.next_value()?;
                Ok(Value::Number(number.value))
            }
            #[cfg(feature = "raw_value")]
            Some(KeyClass::RawValue) => {
                let value = visitor.next_value_seed(crate::raw::BoxedFromString)?;
                crate::from_str(value.get()).map_err(de::Error::custom)
            }
            Some(KeyClass::Map(first_key)) => {
                let mut values = Map::new();

                values.insert(first_key, tri!(visitor.next_value()));
                while let Some((key, value)) = tri!(visitor.next_entry()) {
                    values.insert(key, value);
                }

                Ok(Value::Object(values))
            }
            None => Ok(Value::Object(Map::new())),
        }
    }
}
