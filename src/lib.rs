mod consts;
mod de;
pub mod error;
pub mod integers;
mod read_ext;
mod ser;
mod write_ext;
mod array;

pub use de::{from_reader, Deserializer};
pub use ser::{to_writer, Serializer};
pub use array::OcamlArray;
pub use read_ext::ReadBinProtExt;
pub use write_ext::WriteBinProtExt;
