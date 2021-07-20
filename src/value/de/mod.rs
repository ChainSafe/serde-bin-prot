use std::cell::RefCell;
use std::rc::Rc;
use crate::value::layout::{BranchingIterator, BinProtRuleIterator, BranchIterResult};
use serde::de;
use serde::de::Visitor;
use serde::Deserialize;
use std::io::BufReader;
use std::io::Read;


use crate::value::layout::BinProtRule;
use crate::value::Value;

use crate::error::{Error, Result};


mod visitor;

pub use visitor::ValueVisitor;

use crate::read_ext::ReadBinProtExt;

// impl<'de> Deserialize<'de> for Value {
//     #[inline]
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Value, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         deserializer.deserialize_any(ValueVisitor)
//     }
// }

// pub struct Deserializer<R: Read> {
//     rdr: BufReader<R>,
//     layout_iter: BinProtRuleIterator,
//     stack: Vec<RefCell<Value>>, 
// }

// impl<R: Read> Deserializer<R> {
//     pub fn new(rdr: R, layout: BinProtRule) -> Self {
//         let root = RefCell::new(Value::Placeholder);
//         Self {
//             rdr: BufReader::new(rdr),
//             layout_iter: layout.into_iter(),
//             stack: vec![root], 
//         }
//     }

//     pub fn deserialize(&mut self) -> Value {
//         loop {
//             match self.layout_iter.next() {
//                 BranchIterResult::Item(v) => {
//                     // println!("{:?}\n", v);
//                     match v {
//                         BinProtRule::Int => {
//                             let tip = self.stack.pop().unwrap();
//                             *tip.borrow_mut() = Value::Int(0);
//                         },
//                         BinProtRule::Option(_) => {
//                             let tip = self.stack.pop().unwrap();
//                         }
//                         _ => unimplemented!()
//                     }
//                 }
//                 BranchIterResult::Branch => {
//                     // read which enum variant is present from the stream
//                     //  
//                     // TODO: This is only reading one byte for now but actually this 
//                     // could be two bytes for very large enums - see https://github.com/janestreet/bin_prot#sum-types
//                     let idx = self.rdr.bin_read_variant_index().unwrap();
//                     self.layout_iter.branch(idx.into()).expect("Invalid branch index");
//                 }
//                 BranchIterResult::Err(e) => {
//                     panic!("{}", e);
//                 }
//                 BranchIterResult::End => {
//                     println!("END!!!!");
//                     // return self.root.take();
//                 }
//             }
//         }
//     }
// }

// pub struct Deserializer<R: Read> {
//     rdr: BufReader<R>,
//     iter: BinProtRuleIterator,
// }

// impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
//     type Error = Error;
//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
//     where
//         V: Visitor<'de>,
//     {
//         // Rather than relying on the visitor hints, use the layout
//         // to determine what type to read next
//         match self.iter.next() {
//             BranchIterResult::Item(v) => {
//                 println!("{:?}\n", v);
//             }
//             BranchIterResult::Branch => {
//                 self.iter.branch(0).expect("Invalid branch index");
//             }
//             BranchIterResult::Err(e) => {
//                 panic!("{}", e);
//             }
//             BranchIterResult::End => {
//                 println!("END!!!!");
//             }
//         }
//     }

//     // disregard any type hints and forward everything to deserialize_any to let the 
//     // layout decide how to interpret
//     serde::forward_to_deserialize_any! {
//         bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
//         bytes byte_buf option unit unit_struct newtype_struct seq tuple
//         tuple_struct map struct enum identifier ignored_any
//     }
// }
