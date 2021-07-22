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
fn test_simple_rule() {
    let rule: BinProtRule = serde_json::from_str(SIMPLE_RULE).unwrap();
    let example = vec![0x01, 0x00, 0x00]; // Some((0, false))

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    println!("{:?}", result);
    assert_eq!(
        result,
        Value::Option(Some(Box::new(Value::Tuple(vec![
            Value::Int(0),
            Value::Bool(false)
        ]))))
    )
}

const SUM_RULE: &str = r#"
[
  "Sum",
  [
    {
      "ctor_name": "one",
      "index": 0,
      "ctor_args": [["Int"]]
    },
    {
      "ctor_name": "two",
      "index": 1,
      "ctor_args": [["Bool"]]
    }
  ]
]
"#;

#[test]
fn test_sum_rule() {
    let rule: BinProtRule = serde_json::from_str(SUM_RULE).unwrap();
    let example = vec![0x00, 0x00]; // One((0))

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    println!("{:?}", result);
    assert_eq!(result, Value::Sum{ name: "".to_string(), index: 0, value: Box::new(Value::Int(0)) }) // should be Value::Sum{ name: "one", index: 0, value: Box::new(Value::Int(0)) }
}
