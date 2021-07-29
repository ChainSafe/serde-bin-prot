#![cfg_attr(not(feature = "std"), no_std)]

mod consts;
mod de;
pub mod error;
pub mod integers;
mod read_ext;
mod ser;
mod write_ext;

pub use de::{from_reader, Deserializer};
pub use ser::{to_writer, Serializer};

pub use read_ext::ReadBinProtExt;
pub use write_ext::WriteBinProtExt;
