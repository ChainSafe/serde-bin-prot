# Serde Bin_prot

[![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](http://www.gnu.org/licenses/lgpl-3.0)

A Rust crate that adds serde support for the [Bin_prot](https://github.com/janestreet/bin_prot) serialization format

## Usage

Following Serde convention this crate exposes a `from_reader` and `to_writer` function to serialize/deserialize bin_prot encoded streams of bytes.

Example:

```rust
use serde_bin_prot::{from_reader, to_writer};

fn main() {
  let val: Vec<i64> = vec![20, -22, 38];

  # Example write into a vec of bytes
  let mut output = Vec::<u8>::new();
  to_writer(&mut output, &val).unwrap();

  # Read back into a typed value
  let de_val: Vec<i64> = from_reader(output.as_slice()).unwrap();
  
  assert!(val == de_val)
}

```

## Testing

All tests can be run through cargo

```
cargo test
```

## Licence

Distributed under the GPL-3 License. See LICENSE for more information.

## Contact

{Willem Olding](willem@chainsafe.io)
