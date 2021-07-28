#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use serde::{Deserialize, Serialize};
use serde_bin_prot::integers::{integer, nat0};
mod common {
    use serde::{Deserialize, Serialize};
    use serde_bin_prot::{from_reader, to_writer};
    use std::fmt::Debug;
    use std::fmt::Write;
    /// Prints a byte array according to the style used in the Jane Street
    /// bin_prot tests. Byte array is reversed and padded up to max length with ..
    /// Bytes are written in hex with lowercase letters and no 0x prefix
    pub fn print_byte_array<W: Write>(w: &mut W, bytes: &[u8], max_len: usize) {
        let padding = max_len - bytes.len();
        for _ in 0..padding {
            w.write_fmt(::core::fmt::Arguments::new_v1(
                &[".. "],
                &match () {
                    () => [],
                },
            ))
            .unwrap();
        }
        for b in bytes.iter().rev() {
            w.write_fmt(::core::fmt::Arguments::new_v1_formatted(
                &["", " "],
                &match (&b,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::LowerHex::fmt,
                    )],
                },
                &[::core::fmt::rt::v1::Argument {
                    position: 0usize,
                    format: ::core::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                        flags: 8u32,
                        precision: ::core::fmt::rt::v1::Count::Implied,
                        width: ::core::fmt::rt::v1::Count::Is(2usize),
                    },
                }],
            ))
            .unwrap();
        }
    }
    pub fn roundtrip_test<'a, T: Serialize + Deserialize<'a> + PartialEq + Debug>(val: T) {
        let mut output = Vec::<u8>::new();
        to_writer(&mut output, &val).unwrap();
        let re_val: T = from_reader(output.as_slice()).unwrap();
        {
            match (&val, &re_val) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            }
        }
    }
}
struct TestTupleStruct(bool, i8, i16, i32, i64, (), Option<()>, [u8; 3], char);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for TestTupleStruct {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_tuple_struct(
                __serializer,
                "TestTupleStruct",
                0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.0) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.1) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.2) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.3) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.4) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.5) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.6) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.7) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state, &self.8) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeTupleStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TestTupleStruct {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TestTupleStruct>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TestTupleStruct;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "tuple struct TestTupleStruct",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<i8>(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"tuple struct TestTupleStruct with 9 elements",
                            ));
                        }
                    };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<i16>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    let __field3 =
                        match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    let __field4 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    let __field5 = match match _serde::de::SeqAccess::next_element::<()>(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"tuple struct TestTupleStruct with 9 elements",
                            ));
                        }
                    };
                    let __field6 =
                        match match _serde::de::SeqAccess::next_element::<Option<()>>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    let __field7 =
                        match match _serde::de::SeqAccess::next_element::<[u8; 3]>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    let __field8 =
                        match match _serde::de::SeqAccess::next_element::<char>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    8usize,
                                    &"tuple struct TestTupleStruct with 9 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(TestTupleStruct(
                        __field0, __field1, __field2, __field3, __field4, __field5, __field6,
                        __field7, __field8,
                    ))
                }
            }
            _serde::Deserializer::deserialize_tuple_struct(
                __deserializer,
                "TestTupleStruct",
                9usize,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TestTupleStruct>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl ::core::marker::StructuralPartialEq for TestTupleStruct {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for TestTupleStruct {
    #[inline]
    fn eq(&self, other: &TestTupleStruct) -> bool {
        match *other {
            TestTupleStruct(
                ref __self_1_0,
                ref __self_1_1,
                ref __self_1_2,
                ref __self_1_3,
                ref __self_1_4,
                ref __self_1_5,
                ref __self_1_6,
                ref __self_1_7,
                ref __self_1_8,
            ) => match *self {
                TestTupleStruct(
                    ref __self_0_0,
                    ref __self_0_1,
                    ref __self_0_2,
                    ref __self_0_3,
                    ref __self_0_4,
                    ref __self_0_5,
                    ref __self_0_6,
                    ref __self_0_7,
                    ref __self_0_8,
                ) => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                        && (*__self_0_6) == (*__self_1_6)
                        && (*__self_0_7) == (*__self_1_7)
                        && (*__self_0_8) == (*__self_1_8)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TestTupleStruct) -> bool {
        match *other {
            TestTupleStruct(
                ref __self_1_0,
                ref __self_1_1,
                ref __self_1_2,
                ref __self_1_3,
                ref __self_1_4,
                ref __self_1_5,
                ref __self_1_6,
                ref __self_1_7,
                ref __self_1_8,
            ) => match *self {
                TestTupleStruct(
                    ref __self_0_0,
                    ref __self_0_1,
                    ref __self_0_2,
                    ref __self_0_3,
                    ref __self_0_4,
                    ref __self_0_5,
                    ref __self_0_6,
                    ref __self_0_7,
                    ref __self_0_8,
                ) => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                        || (*__self_0_6) != (*__self_1_6)
                        || (*__self_0_7) != (*__self_1_7)
                        || (*__self_0_8) != (*__self_1_8)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for TestTupleStruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            TestTupleStruct(
                ref __self_0_0,
                ref __self_0_1,
                ref __self_0_2,
                ref __self_0_3,
                ref __self_0_4,
                ref __self_0_5,
                ref __self_0_6,
                ref __self_0_7,
                ref __self_0_8,
            ) => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_tuple(f, "TestTupleStruct");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_2));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_3));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_4));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_5));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_6));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_7));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_8));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
struct A(bool);
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for A {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "A", &self.0)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for A {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<A>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = A;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "tuple struct A")
                }
                #[inline]
                fn visit_newtype_struct<__E>(
                    self,
                    __e: __E,
                ) -> _serde::__private::Result<Self::Value, __E::Error>
                where
                    __E: _serde::Deserializer<'de>,
                {
                    let __field0: bool = match <bool as _serde::Deserialize>::deserialize(__e) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::__private::Ok(A(__field0))
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct A with 1 element",
                                ));
                            }
                        };
                    _serde::__private::Ok(A(__field0))
                }
            }
            _serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                "A",
                __Visitor {
                    marker: _serde::__private::PhantomData::<A>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl ::core::marker::StructuralPartialEq for A {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for A {
    #[inline]
    fn eq(&self, other: &A) -> bool {
        match *other {
            A(ref __self_1_0) => match *self {
                A(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &A) -> bool {
        match *other {
            A(ref __self_1_0) => match *self {
                A(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            A(ref __self_0_0) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "A");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
struct B {
    a: A,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for B {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state =
                match _serde::Serializer::serialize_struct(__serializer, "B", false as usize + 1) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "a", &self.a) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for B {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "a" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"a" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<B>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = B;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct B")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<A>(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct B with 1 element",
                            ));
                        }
                    };
                    _serde::__private::Ok(B { a: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<A> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<A>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("a") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(B { a: __field0 })
                }
            }
            const FIELDS: &'static [&'static str] = &["a"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "B",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<B>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl ::core::marker::StructuralPartialEq for B {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for B {
    #[inline]
    fn eq(&self, other: &B) -> bool {
        match *other {
            B { a: ref __self_1_0 } => match *self {
                B { a: ref __self_0_0 } => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &B) -> bool {
        match *other {
            B { a: ref __self_1_0 } => match *self {
                B { a: ref __self_0_0 } => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for B {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            B { a: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "B");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "a", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
struct TestFieldAttrs {
    #[serde(with = "nat0")]
    n: u8,
    #[serde(with = "integer")]
    i: i16,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for TestFieldAttrs {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "TestFieldAttrs",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "n", {
                struct __SerializeWith<'__a> {
                    values: (&'__a u8,),
                    phantom: _serde::__private::PhantomData<TestFieldAttrs>,
                }
                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                    fn serialize<__S>(
                        &self,
                        __s: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        nat0::serialize(self.values.0, __s)
                    }
                }
                &__SerializeWith {
                    values: (&self.n,),
                    phantom: _serde::__private::PhantomData::<TestFieldAttrs>,
                }
            }) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "i", {
                struct __SerializeWith<'__a> {
                    values: (&'__a i16,),
                    phantom: _serde::__private::PhantomData<TestFieldAttrs>,
                }
                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                    fn serialize<__S>(
                        &self,
                        __s: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        integer::serialize(self.values.0, __s)
                    }
                }
                &__SerializeWith {
                    values: (&self.i,),
                    phantom: _serde::__private::PhantomData::<TestFieldAttrs>,
                }
            }) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TestFieldAttrs {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "n" => _serde::__private::Ok(__Field::__field0),
                        "i" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"n" => _serde::__private::Ok(__Field::__field0),
                        b"i" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TestFieldAttrs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TestFieldAttrs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TestFieldAttrs")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match {
                        struct __DeserializeWith<'de> {
                            value: u8,
                            phantom: _serde::__private::PhantomData<TestFieldAttrs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::__private::Ok(__DeserializeWith {
                                    value: match nat0::deserialize(__deserializer) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                    phantom: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                })
                            }
                        }
                        _serde::__private::Option::map(
                            match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                            |__wrap| __wrap.value,
                        )
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct TestFieldAttrs with 2 elements",
                            ));
                        }
                    };
                    let __field1 = match {
                        struct __DeserializeWith<'de> {
                            value: i16,
                            phantom: _serde::__private::PhantomData<TestFieldAttrs>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::__private::Ok(__DeserializeWith {
                                    value: match integer::deserialize(__deserializer) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                    phantom: _serde::__private::PhantomData,
                                    lifetime: _serde::__private::PhantomData,
                                })
                            }
                        }
                        _serde::__private::Option::map(
                            match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                &mut __seq,
                            ) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                            |__wrap| __wrap.value,
                        )
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct TestFieldAttrs with 2 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(TestFieldAttrs {
                        n: __field0,
                        i: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("n"),
                                    );
                                }
                                __field0 = _serde::__private::Some({
                                    struct __DeserializeWith<'de> {
                                        value: u8,
                                        phantom: _serde::__private::PhantomData<TestFieldAttrs>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::__private::Ok(__DeserializeWith {
                                                value: match nat0::deserialize(__deserializer) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                                phantom: _serde::__private::PhantomData,
                                                lifetime: _serde::__private::PhantomData,
                                            })
                                        }
                                    }
                                    match _serde::de::MapAccess::next_value::<__DeserializeWith<'de>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__wrapper) => __wrapper.value,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                });
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("i"),
                                    );
                                }
                                __field1 = _serde::__private::Some({
                                    struct __DeserializeWith<'de> {
                                        value: i16,
                                        phantom: _serde::__private::PhantomData<TestFieldAttrs>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::__private::Ok(__DeserializeWith {
                                                value: match integer::deserialize(__deserializer) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                                phantom: _serde::__private::PhantomData,
                                                lifetime: _serde::__private::PhantomData,
                                            })
                                        }
                                    }
                                    match _serde::de::MapAccess::next_value::<__DeserializeWith<'de>>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__wrapper) => __wrapper.value,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                });
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("n"),
                            )
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("i"),
                            )
                        }
                    };
                    _serde::__private::Ok(TestFieldAttrs {
                        n: __field0,
                        i: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["n", "i"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "TestFieldAttrs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TestFieldAttrs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl ::core::marker::StructuralPartialEq for TestFieldAttrs {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for TestFieldAttrs {
    #[inline]
    fn eq(&self, other: &TestFieldAttrs) -> bool {
        match *other {
            TestFieldAttrs {
                n: ref __self_1_0,
                i: ref __self_1_1,
            } => match *self {
                TestFieldAttrs {
                    n: ref __self_0_0,
                    i: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TestFieldAttrs) -> bool {
        match *other {
            TestFieldAttrs {
                n: ref __self_1_0,
                i: ref __self_1_1,
            } => match *self {
                TestFieldAttrs {
                    n: ref __self_0_0,
                    i: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for TestFieldAttrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            TestFieldAttrs {
                n: ref __self_0_0,
                i: ref __self_0_1,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "TestFieldAttrs");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "n", &&(*__self_0_0));
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "i", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
pub struct CompressedPoly {
    version: u8,
    x: [u8; 32],
    is_odd: bool,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for CompressedPoly {
    #[inline]
    fn clone(&self) -> CompressedPoly {
        match *self {
            CompressedPoly {
                version: ref __self_0_0,
                x: ref __self_0_1,
                is_odd: ref __self_0_2,
            } => CompressedPoly {
                version: ::core::clone::Clone::clone(&(*__self_0_0)),
                x: ::core::clone::Clone::clone(&(*__self_0_1)),
                is_odd: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CompressedPoly {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "CompressedPoly",
                false as usize + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "version",
                &self.version,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "x", &self.x) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "is_odd",
                &self.is_odd,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CompressedPoly {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "version" => _serde::__private::Ok(__Field::__field0),
                        "x" => _serde::__private::Ok(__Field::__field1),
                        "is_odd" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"version" => _serde::__private::Ok(__Field::__field0),
                        b"x" => _serde::__private::Ok(__Field::__field1),
                        b"is_odd" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CompressedPoly>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CompressedPoly;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct CompressedPoly")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct CompressedPoly with 3 elements",
                            ));
                        }
                    };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<[u8; 32]>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CompressedPoly with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct CompressedPoly with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(CompressedPoly {
                        version: __field0,
                        x: __field1,
                        is_odd: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<[u8; 32]> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "version",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("x"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<[u8; 32]>(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "is_odd",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("version") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("x") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("is_odd") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(CompressedPoly {
                        version: __field0,
                        x: __field1,
                        is_odd: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["version", "x", "is_odd"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CompressedPoly",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CompressedPoly>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for CompressedPoly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            CompressedPoly {
                version: ref __self_0_0,
                x: ref __self_0_1,
                is_odd: ref __self_0_2,
            } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "CompressedPoly");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "version",
                    &&(*__self_0_0),
                );
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "x", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "is_odd", &&(*__self_0_2));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for CompressedPoly {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for CompressedPoly {
    #[inline]
    fn eq(&self, other: &CompressedPoly) -> bool {
        match *other {
            CompressedPoly {
                version: ref __self_1_0,
                x: ref __self_1_1,
                is_odd: ref __self_1_2,
            } => match *self {
                CompressedPoly {
                    version: ref __self_0_0,
                    x: ref __self_0_1,
                    is_odd: ref __self_0_2,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &CompressedPoly) -> bool {
        match *other {
            CompressedPoly {
                version: ref __self_1_0,
                x: ref __self_1_1,
                is_odd: ref __self_1_2,
            } => match *self {
                CompressedPoly {
                    version: ref __self_0_0,
                    x: ref __self_0_1,
                    is_odd: ref __self_0_2,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
pub enum E {
    A,
    B,
    C,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for E {
    #[inline]
    fn clone(&self) -> E {
        match (&*self,) {
            (&E::A,) => E::A,
            (&E::B,) => E::B,
            (&E::C,) => E::C,
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for E {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                E::A => _serde::Serializer::serialize_unit_variant(__serializer, "E", 0u32, "A"),
                E::B => _serde::Serializer::serialize_unit_variant(__serializer, "E", 1u32, "B"),
                E::C => _serde::Serializer::serialize_unit_variant(__serializer, "E", 2u32, "C"),
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for E {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 3",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "A" => _serde::__private::Ok(__Field::__field0),
                        "B" => _serde::__private::Ok(__Field::__field1),
                        "C" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"A" => _serde::__private::Ok(__Field::__field0),
                        b"B" => _serde::__private::Ok(__Field::__field1),
                        b"C" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<E>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = E;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum E")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(E::A)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(E::B)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(E::C)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["A", "B", "C"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "E",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<E>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for E {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&E::A,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "A");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&E::B,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "B");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&E::C,) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "C");
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for E {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for E {
    #[inline]
    fn eq(&self, other: &E) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
pub enum EnumWithValues {
    A(bool),
    B(i32, f32),
    C(Option<bool>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for EnumWithValues {
    #[inline]
    fn clone(&self) -> EnumWithValues {
        match (&*self,) {
            (&EnumWithValues::A(ref __self_0),) => {
                EnumWithValues::A(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&EnumWithValues::B(ref __self_0, ref __self_1),) => EnumWithValues::B(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&EnumWithValues::C(ref __self_0),) => {
                EnumWithValues::C(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for EnumWithValues {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                EnumWithValues::A(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                    __serializer,
                    "EnumWithValues",
                    0u32,
                    "A",
                    __field0,
                ),
                EnumWithValues::B(ref __field0, ref __field1) => {
                    let mut __serde_state = match _serde::Serializer::serialize_tuple_variant(
                        __serializer,
                        "EnumWithValues",
                        1u32,
                        "B",
                        0 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field0,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeTupleVariant::serialize_field(
                        &mut __serde_state,
                        __field1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeTupleVariant::end(__serde_state)
                }
                EnumWithValues::C(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                    __serializer,
                    "EnumWithValues",
                    2u32,
                    "C",
                    __field0,
                ),
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for EnumWithValues {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 3",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "A" => _serde::__private::Ok(__Field::__field0),
                        "B" => _serde::__private::Ok(__Field::__field1),
                        "C" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"A" => _serde::__private::Ok(__Field::__field0),
                        b"B" => _serde::__private::Ok(__Field::__field1),
                        b"C" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<EnumWithValues>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = EnumWithValues;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum EnumWithValues")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => _serde::__private::Result::map(
                            _serde::de::VariantAccess::newtype_variant::<bool>(__variant),
                            EnumWithValues::A,
                        ),
                        (__Field::__field1, __variant) => {
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<EnumWithValues>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = EnumWithValues;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result
                                {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "tuple variant EnumWithValues::B",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        i32,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (0usize , & "tuple variant EnumWithValues::B with 2 elements")) ;
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        f32,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde :: __private :: Err (_serde :: de :: Error :: invalid_length (1usize , & "tuple variant EnumWithValues::B with 2 elements")) ;
                                        }
                                    };
                                    _serde::__private::Ok(EnumWithValues::B(__field0, __field1))
                                }
                            }
                            _serde::de::VariantAccess::tuple_variant(
                                __variant,
                                2usize,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<EnumWithValues>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field2, __variant) => _serde::__private::Result::map(
                            _serde::de::VariantAccess::newtype_variant::<Option<bool>>(__variant),
                            EnumWithValues::C,
                        ),
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &["A", "B", "C"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "EnumWithValues",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<EnumWithValues>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for EnumWithValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&EnumWithValues::A(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "A");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&EnumWithValues::B(ref __self_0, ref __self_1),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "B");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_1));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&EnumWithValues::C(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "C");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for EnumWithValues {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for EnumWithValues {
    #[inline]
    fn eq(&self, other: &EnumWithValues) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&EnumWithValues::A(ref __self_0), &EnumWithValues::A(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &EnumWithValues::B(ref __self_0, ref __self_1),
                        &EnumWithValues::B(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (&EnumWithValues::C(ref __self_0), &EnumWithValues::C(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &EnumWithValues) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&EnumWithValues::A(ref __self_0), &EnumWithValues::A(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &EnumWithValues::B(ref __self_0, ref __self_1),
                        &EnumWithValues::B(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (&EnumWithValues::C(ref __self_0), &EnumWithValues::C(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
pub struct PublicKey {
    version: u8,
    poly: CompressedPoly,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for PublicKey {
    #[inline]
    fn clone(&self) -> PublicKey {
        match *self {
            PublicKey {
                version: ref __self_0_0,
                poly: ref __self_0_1,
            } => PublicKey {
                version: ::core::clone::Clone::clone(&(*__self_0_0)),
                poly: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for PublicKey {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "PublicKey",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "version",
                &self.version,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "poly",
                &self.poly,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for PublicKey {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "version" => _serde::__private::Ok(__Field::__field0),
                        "poly" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"version" => _serde::__private::Ok(__Field::__field0),
                        b"poly" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PublicKey>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PublicKey;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PublicKey")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<u8>(&mut __seq)
                    {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct PublicKey with 2 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<CompressedPoly>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct PublicKey with 2 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(PublicKey {
                        version: __field0,
                        poly: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<CompressedPoly> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "version",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("poly"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<CompressedPoly>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("version") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("poly") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(PublicKey {
                        version: __field0,
                        poly: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["version", "poly"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PublicKey",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PublicKey>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            PublicKey {
                version: ref __self_0_0,
                poly: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "PublicKey");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "version",
                    &&(*__self_0_0),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "poly", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for PublicKey {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for PublicKey {
    #[inline]
    fn eq(&self, other: &PublicKey) -> bool {
        match *other {
            PublicKey {
                version: ref __self_1_0,
                poly: ref __self_1_1,
            } => match *self {
                PublicKey {
                    version: ref __self_0_0,
                    poly: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &PublicKey) -> bool {
        match *other {
            PublicKey {
                version: ref __self_1_0,
                poly: ref __self_1_1,
            } => match *self {
                PublicKey {
                    version: ref __self_0_0,
                    poly: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
impl PublicKey {
    pub fn new() -> Self {
        PublicKey {
            version: 1,
            poly: CompressedPoly {
                version: 1,
                x: [0x0; 32],
                is_odd: false,
            },
        }
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const roundtrip_tuple_struct: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("roundtrip_tuple_struct"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(roundtrip_tuple_struct())),
};
fn roundtrip_tuple_struct() {
    common::roundtrip_test(TestTupleStruct(
        true,
        0,
        0,
        0,
        0,
        (),
        None,
        [0x01, 0x02, 0x03],
        'c',
    ));
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const roundtrip_array_in_struct: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("roundtrip_array_in_struct"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(roundtrip_array_in_struct())),
};
fn roundtrip_array_in_struct() {
    common::roundtrip_test(PublicKey::new())
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const roundtrip_nested_structs: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("roundtrip_nested_structs"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(roundtrip_nested_structs())),
};
fn roundtrip_nested_structs() {
    common::roundtrip_test(B { a: A(false) });
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const roundtrip_enum: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("roundtrip_enum"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(roundtrip_enum())),
};
fn roundtrip_enum() {
    common::roundtrip_test(E::A);
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const roundtrip_enum_with_values: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("roundtrip_enum_with_values"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(roundtrip_enum_with_values())),
};
fn roundtrip_enum_with_values() {
    common::roundtrip_test(EnumWithValues::C(Some(true)));
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const roundtrip_owned_string: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("roundtrip_owned_string"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(roundtrip_owned_string())),
};
fn roundtrip_owned_string() {
    common::roundtrip_test("serde-bin-prot".to_string());
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &roundtrip_tuple_struct,
        &roundtrip_array_in_struct,
        &roundtrip_nested_structs,
        &roundtrip_enum,
        &roundtrip_enum_with_values,
        &roundtrip_owned_string,
    ])
}
