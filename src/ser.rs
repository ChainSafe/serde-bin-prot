use crate::consts::*;
use crate::error::{Error, Result};
use serde::{ser, Serialize};

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Serializer<W>
where
    W: std::io::Write,
{
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.writer.write_all(buf)?;
        Ok(())
    }

    pub fn write_byte(&mut self, b: u8) -> Result<()> {
        self.write(&[b])
    }
}

pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: Serialize,
{
    value.serialize(&mut Serializer::new(writer))
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // the spec defines booleans be written as:
    // false  ->  0x00
    // true   ->  0x01
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write_byte(if v { 0x00 } else { 0x01 })
    }

    // The little-endian format is used in the protocol for the contents of integers on all platforms

    // For all Integer types:
    // if the value is positive (including zero) and if it is:
    // <  0x00000080  ->  lower 8 bits of the integer                     (1 byte)
    // <  0x00008000  ->  CODE_INT16 followed by lower 16 bits of integer (3 bytes)
    // <  0x80000000  ->  CODE_INT32 followed by lower 32 bits of integer (5 bytes)
    // >= 0x80000000  ->  CODE_INT64 followed by all 64 bits of integer   (9 bytes)

    // If the value is negative and if it is:
    // >= -0x00000080  ->  CODE_NEG_INT8 followed by lower 8 bits of integer (2 bytes)
    // >= -0x00008000  ->  CODE_INT16 followed by lower 16 bits of integer   (3 bytes)
    // >= -0x80000000  ->  CODE_INT32 followed by lower 32 bits of integer   (5 bytes)
    // <  -0x80000000  ->  CODE_INT64 followed by all 64 bits of integer     (9 bytes)

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v.into())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(v.into())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        // this is where all the integer serialization logic lives
        if v >= 0 { // positive or zero case
            match v {
                _ if v < 0x00000080 => { self.write(&v.to_le_bytes()[..1]) }
                _ if v < 0x00008000 => { self.write_byte(CODE_INT16)?; self.write(&v.to_le_bytes()[..2]) }
                _ if v < 0x80000000 => { self.write_byte(CODE_INT32)?; self.write(&v.to_le_bytes()[..4]) }
                _ => { self.write_byte(CODE_INT64)?; self.write(&v.to_le_bytes()) }
            }
        } else { // negative case
            match v {
                _ if v >= -0x00000080 => { self.write_byte(CODE_NEG_INT8)?; self.write(&v.to_le_bytes()[..1]) }
                _ if v >= -0x00008000 => { self.write_byte(CODE_INT16)?; self.write(&v.to_le_bytes()[..2]) }
                _ if v >= -0x80000000 => { self.write_byte(CODE_INT32)?; self.write(&v.to_le_bytes()[..4]) }
                _ => { self.write_byte(CODE_INT64)?; self.write(&v.to_le_bytes()) }
            }
        }
    }

    // Unsigned forms just cast to signed forms then serialize
    // TODO: There could probably be some optimization done here
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    // Floats are written out according to the 64 bit IEEE 754 floating point standard
    // i.e. their memory representation (in OCaml) is copied verbatim.
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        todo!()
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<()> {
        todo!()
    }

    // just treat this like any other array for now
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write(v)
        // use serde::ser::SerializeSeq;
        // let mut seq = self.serialize_seq(Some(v.len()))?;
        // for byte in v {
        //     seq.serialize_element(byte)?;
        // }
        // seq.end()
    }

    // An absent optional is represented as a unit or zero byte
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    // A present optional is represented as a 0x01 byte
    // followed by the encoding of the value
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_byte(0x01)?;
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data.
    // This is a zero byte
    fn serialize_unit(self) -> Result<()> {
        self.write_byte(0x00)?;
        Ok(())
    }

    // Unit struct means a named value containing no data.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    // TODO: What even is this?
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // TODO: What even is this?
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    // Now we get to the serialization of compound types.
    //
    // For lists and arrays the length is written out as a Nat0.t first,
    // followed by all values in the same order as in the data structure.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    // Values in tuples and records are written out one after the other in the order
    // specified in the type definition.
    // Polymorphic record fields are supported unless a value of the type bound
    // by the field were accessed, which would lead to an exception.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: std::io::Write, // Must match the `Ok` type of the serializer.
{
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        todo!()
    }
}

// Same thing but for tuples.
impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Same thing but for tuple structs.
impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}
