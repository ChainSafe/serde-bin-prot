
use serde::de::Deserialize;
use serde_bin_prot::value::layout::BinProtRule;
use serde_bin_prot::value::Value;
use serde_bin_prot::Deserializer;

const SIMPLE_RULE: &str = r#"
[
  "Option",
  [
    "Tuple",
    [
      ["Int"],
      ["Bool"]
    ]
  ]
]

"#;


#[test]
fn test_layouts() {
    let rule: BinProtRule = serde_json::from_str(SIMPLE_RULE).unwrap();
    let example = vec![0x01, 0x00, 0x00]; // Some((0, false))
    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let value: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    println!("{:?}", value)
}
