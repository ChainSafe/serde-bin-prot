use serde_bin_prot::value::layout::BranchingIterator;
use serde::de::Deserialize;
use serde_bin_prot::value::layout::{BinProtRule, Layout};
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

const RECORD_RULE: &str = r#"
[
  "Record",
  [
    { "field_name": "first", "field_rule": ["Int"] },
    { "field_name": "second", "field_rule": ["Record", [{ "field_name": "inner", "field_rule": ["Bool"] }] ] }
  ]
]
"#;

#[test]
fn test_record_rule() {
    let rule: BinProtRule = serde_json::from_str(RECORD_RULE).unwrap();
    let example = vec![0x00, 0x01];

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    println!("{:?}", result);
    assert_eq!(
        result,
        Value::Record(vec![
            ("first".to_string(), Value::Int(0)),
            (
                "second".to_string(),
                Value::Record(vec![("inner".to_string(), Value::Bool(true))])
            )
        ])
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
    let example = vec![0x01, 0x00]; // Two((false))

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(
        result,
        Value::Sum {
            name: "two".to_string(),
            index: 1,
            value: Box::new(Value::Bool(false))
        }
    )
}

const NESTED_SUM_RULE: &str = r#"
[
  "Sum",
  [
    {
      "ctor_name": "one",
      "index": 0,
      "ctor_args": [[
        "Record",
          [
            { "field_name": "first", "field_rule": ["Int"] }
          ]
        ]
       ]
    }
  ]
]
"#;

#[test]
fn test_nested_sum_rule() {
    let rule: BinProtRule = serde_json::from_str(NESTED_SUM_RULE).unwrap();
    let example = vec![0x00, 0x05]; // One({ first: 5 })

    let mut de = Deserializer::from_reader_with_layout(example.as_slice(), rule);
    let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
    assert_eq!(
        result,
        Value::Sum {
            name: "one".to_string(),
            index: 0,
            value: Box::new(Value::Record(vec![("first".to_string(), Value::Int(5))]))
        }
    )
}

const BLOCK_LAYOUT: &str = std::include_str!("external_transition_customs.json");
// const BLOCK_BYTES: &[u8] = std::include_bytes!("block.hex");

// #[test]
// fn test_block() {
//     let mut deserializer = serde_json::Deserializer::from_str(BLOCK_LAYOUT);
//     deserializer.disable_recursion_limit();
//     let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
//     let rule = Layout::deserialize(deserializer).unwrap().bin_prot_rule;

//     let mut de = Deserializer::from_reader_with_layout(BLOCK_BYTES, rule);
//     let result: Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");
// }

#[test]
fn test_find_custom_rules() {
    let mut deserializer = serde_json::Deserializer::from_str(BLOCK_LAYOUT);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let rule = Layout::deserialize(deserializer).unwrap().bin_prot_rule;

    let mut custom_rules = std::collections::HashSet::<String>::new();

    let mut iter = rule.into_branching_iter();
    // Test by taking the 0th branch at each branch node. Test is considered as pass
    // if no error
    loop {
        match iter.next() {
            Ok(Some(v)) => {
                if let BinProtRule::Sum(_) | BinProtRule::Polyvar(_) = v {
                    // if its a sum type take the first variant in each case
                    iter.branch(0).expect("Invalid branch index");
                }
                if let BinProtRule::CustomForPath(path) = v {
                    custom_rules.insert(path);
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
            Ok(None) => {
                println!("END!!!!");
                break;
            }
        }
    }

    println!("{:?}", custom_rules);

}
