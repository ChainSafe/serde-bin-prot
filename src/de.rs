
use crate::error::{Error, ErrorCode, Result};
use crate::integers::ReadBinProtIntegerExt;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::{self, Error as DeError, Visitor};
use serde::Deserialize;
use std::io::{BufReader, Read};

pub struct Deserializer<R: Read> {
    rdr: BufReader<R>,
    pos: usize,
}

impl<R: Read> Deserializer<R> {
    fn from_reader(rdr: R) -> Self {
        Self {
            rdr: BufReader::new(rdr),
            pos: 0,
        }
    }

    fn error<T>(&self, reason: ErrorCode) -> Result<T> {
        Err(Error::Eval(reason, self.pos))
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8> {
        let mut buf = [0];
        match self.rdr.read(&mut buf) {
            Ok(1) => {
                self.pos += 1;
                Ok(buf[0])
            }
            Ok(_) => self.error(ErrorCode::EOFWhileParsing),
            Err(err) => Err(Error::Io(err)),
        }
    }

    #[inline]
    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        match self.rdr.by_ref().take(n as u64).read_to_end(&mut buf) {
            Ok(m) if n == m => {
                self.pos += n;
                Ok(buf)
            }
            Ok(_) => self.error(ErrorCode::EOFWhileParsing),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

pub fn from_reader<'de, R: Read, T: Deserialize<'de>>(rdr: R) -> Result<T> {
    let mut de = Deserializer::from_reader(rdr);
    let value = Deserialize::deserialize(&mut de)?;
    Ok(value)
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // bools are a single byte 0x00 or 0x01
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.read_byte()? {
            0x00 => visitor.visit_bool(false),
            0x01 => visitor.visit_bool(true),
            _ => Err(Error::custom("Invalid bool byte")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.rdr.read_binprot_integer()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.rdr.read_f32::<LittleEndian>()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.rdr.read_f64::<LittleEndian>()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // An absent optional is represented as 0x00
    // A present optional is 0x01 followed by the encoded value
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.read_byte()? {
            0x00 => visitor.visit_none(),
            0x01 => visitor.visit_some(self),
            _ => Err(Error::custom("Invalid bool byte")),
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.read_byte()? {
            0x00 => visitor.visit_unit(),
            _ => Err(Error::custom("Invalid unit byte")),
        }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Parsing an unknown length seq (e.g array, lust) involves
    // first reading the length as a Nat0 and then parsing the next values
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.rdr.read_binprot_nat0()?;
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    // Tuples look just like sequences
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    // Tuple structs look just like sequences
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    // Structs look just like sequences
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, fields.len()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqAccess<'a, R: Read + 'a> {
    de: &'a mut Deserializer<R>,
    len: usize,
}

impl<'a, R: Read + 'a> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, len: usize) -> Self {
        Self {
            de,
            len
        }
    }
}

impl<'de: 'a, 'a, R: Read> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.len > 0 {
            self.len -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}
