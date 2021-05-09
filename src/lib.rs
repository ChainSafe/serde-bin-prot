#![cfg_attr(not(feature = "std"), no_std)]

mod consts;
mod de;
mod error;
mod integers;
mod nat0;
mod ser;

pub use de::{from_reader, Deserializer};
pub use ser::{to_writer, Serializer};
pub use error::{Error, Result};

pub use nat0::Nat0;
