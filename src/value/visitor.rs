use crate::value::Value;
use serde::de::EnumAccess;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;

pub struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any valid OCaml value")
    }

    #[inline]
    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    #[inline]
    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Int(value))
    }

    // #[inline]
    // fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
    //     Ok(Value::Int(value.into()))
    // }

    #[inline]
    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
        Ok(Value::Float(value))
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(String::from(value))
    }

    #[inline]
    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Option(None))
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Value::Option(Some(Box::new(Deserialize::deserialize(
            deserializer,
        )?))))
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
        Ok(Value::Tuple(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some((k, v)) = visitor.next_entry()? {
            values.push((k, v))
        }
        Ok(Value::Record(values))
    }

    fn visit_enum<A>(self, visitor: A) -> Result<Self::Value, A::Error> where
        A: EnumAccess<'de>,
    {
        let (v, _variant_visitor) = visitor.variant()?;

        // TODO: Figure out how to get the name, index and value of the variant
        Ok(Value::Sum {
            name: "".to_string(),
            index: v,
            value: Box::new(Value::Unit)
        })
    }
}
