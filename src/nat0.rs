use shrinkwraprs::Shrinkwrap;
use serde::{Serialize, Serializer};
use crate::consts::*;

// This type encodes natural numbers including zero. It is frequently used by Bin_prot itself for encoding size information for e.g. lists, arrays, etc., and hence defined first here. Developers can reuse this type in their code, too, of course.
// 
// If the value of the underlying integer is lower than a certain range, this implies a certain encoding as provided on the right hand side of the following definitions:
// 
// :::text
// <  0x000000080  ->  lower 8 bits of the integer                     (1 byte)
// <  0x000010000  ->  CODE_INT16 followed by lower 16 bits of integer (3 bytes)
// <  0x100000000  ->  CODE_INT32 followed by lower 32 bits of integer (5 bytes)
// >= 0x100000000  ->  CODE_INT64 followed by all 64 bits of integer   (9 bytes)

#[derive(Shrinkwrap, Debug)]
pub struct Nat0(u64);

impl Nat0 {
	pub fn new(v: u64) -> Self {
		Self(v)
	}
}

impl Serialize for Nat0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
    	let v = self.0;
        match v {
            _ if v < 0x000000080 => { serializer.serialize_bytes(&v.to_le_bytes()[..1]) }
            _ if v < 0x000010000 => { serializer.serialize_bytes(&[&[CODE_INT16], &v.to_le_bytes()[..2]].concat()) }
            _ if v < 0x100000000 => { serializer.serialize_bytes(&[&[CODE_INT32], &v.to_le_bytes()[..4]].concat()) }
            _ => { serializer.serialize_bytes(&[&[CODE_INT64], &v.to_le_bytes()[..]].concat()) }
        }     
    }
}

impl From<usize> for Nat0 {
    fn from(v: usize) -> Self {
        Self(v as u64)
    }
}
