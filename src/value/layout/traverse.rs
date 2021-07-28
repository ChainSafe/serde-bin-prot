//!
//! This module implements the logic for traversing a layout in a way that corresponds to how the binary data is laid out.
//! Since BinProtRule is a recursive data type the correct traversal is a depth first traversal of the tree defined by the data type
//! with one important difference.
//!
//! The layout tree includes the concept of Sum(ocaml)/Enum(rust) types. These are nodes in the tree where only one branch should be taken
//! depending on which variant of the enum we are deserializing. When reading an enum from binary the first byte specifies which variant to deserialize.
//! Therefore out DFS iterator requires a way to accept input and branch at particular nodes in the tree.
//!
//! This is where the BranchingIterator trait comes in. A branching iterator is similar to a regular iterator but at some points in its iteration it requires
//! the calling code call a `branch` function to tell it which path to take.
//!
//! The traversal of a layout should be done in parallel with reading a bin_prot encoded binary such that the type tree informs the deserializer how to read the
//! data and the data informs the traversal how it should handle enum types.
//! Combined this should allow parsing of types defined by the layout into loosely typed representations.
//!

use crate::value::layout::{BinProtRule, RuleRef, Summand};

/// Implements a depth first search of the type tree
/// defined by a BinProtRule
pub struct BinProtRuleIterator {
    stack: Vec<BinProtRule>, // regular stack to implement the DFS
    // Tree nodes can branch (only one child should be followed) rather than require traversal of all children
    // If that is the case the parent should add the children to the branch field and the next path will be taken from here rather than the stack
    branch: Option<Vec<Summand>>,
    current_module_path: Option<String>, // holds on to most recent path encountered in traverse
}

/// An iterator where the next item may require specifying a branch to take
/// At some points in its iteration the iterator will require a call to branch to select which
/// path to take before continuing
pub trait BranchingIterator {
    type Item;
    type Error;
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;
    fn branch(&mut self, branch: usize) -> Result<(), Self::Error>;
}

impl BranchingIterator for BinProtRuleIterator {
    type Item = BinProtRule;
    type Error = String;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if self.branch.is_some() {
            return Err("Must call branch to proceed".to_string());
        }

        let top = self.stack.pop();
        let r = top.clone();
        match top {
            Some(rule) => {
                match rule {
                    BinProtRule::Option(r) | BinProtRule::List(r) => {
                        self.stack.push(*r);
                    }
                    BinProtRule::Record(mut rules) => {
                        self.stack
                            .extend(rules.drain(0..).map(|field| field.field_rule).rev());
                    }
                    BinProtRule::Tuple(mut rules) => {
                        self.stack.extend(rules.drain(0..).rev());
                    }
                    BinProtRule::Sum(summands) => {
                        // don't add to the stack. Add to the branch field instead
                        // this must be resolved by calling `branch` before the iterator can continue
                        self.branch = Some(summands.to_vec());
                    }
                    BinProtRule::Reference(rule_ref) => match rule_ref {
                        RuleRef::Unresolved(_payload) => {
                            unimplemented!();
                        }
                        RuleRef::Resolved(payload) => {
                            self.stack.push(*payload.ref_rule);
                            self.current_module_path = Some(payload.source_module_path);
                        }
                    },
                    BinProtRule::String
                    | BinProtRule::Int
                    | BinProtRule::Int64
                    | BinProtRule::Nat0
                    | BinProtRule::Bool
                    | BinProtRule::Unit
                    | BinProtRule::Char
                    | BinProtRule::Int32
                    | BinProtRule::NativeInt
                    | BinProtRule::Float => {} // These are leaves so nothing required
                    BinProtRule::Custom => {
                        if let Some(path) = &self.current_module_path {
                            return Ok(Some(BinProtRule::CustomForPath(path.to_string())));
                        }
                    }
                    r => panic!("unimplemented: {:?}", r),
                };
                Ok(r)
            }
            None => Ok(None), // end of traversal
        }
    }

    fn branch(&mut self, branch: usize) -> Result<(), Self::Error> {
        if let Some(summands) = &self.branch {
            if branch >= summands.len() {
                return Err(format!(
                    "Invalid branch index. Must be < {}",
                    summands.len()
                ));
            }
        }

        if let Some(mut summands) = self.branch.take() {
            // check this is the right way around...
            let s = summands
                .get_mut(branch)
                .ok_or_else(|| "Invalid branch".to_string())?;
            self.stack.extend(s.ctor_args.drain(0..));
            Ok(())
        } else {
            Err("Cannot branch at this location in the tree".to_string())
        }
    }
}

// Consumes the rule and produces a branching iterator
impl BinProtRule {
    pub fn into_branching_iter(self) -> BinProtRuleIterator {
        BinProtRuleIterator {
            stack: vec![self],
            branch: None,
            current_module_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::layout::Layout;
    const TEST_LAYOUT: &str = r#"
{
  "layout_loc":
    "File \"src/lib/mina_networking/mina_networking.ml\", line 321, characters 8-143:",
  "version_opt": null,
  "type_decl":
    "type response = (State_hash.Stable.V1.t * State_body_hash.Stable.V1.t list) option",
  "bin_io_derived": true,
  "bin_prot_rule": [
    "Option",
    [
      "Tuple",
      [
        [
          "Reference",
          [
            "Resolved",
            {
              "source_type_decl": "type t = { version: int ; t: typ }",
              "ref_rule": [
                "Record",
                [
                  { "field_name": "version", "field_rule": [ "Int" ] },
                  {
                    "field_name": "t",
                    "field_rule": [
                      "Reference",
                      [
                        "Resolved",
                        {
                          "source_type_decl": "type typ = t",
                          "ref_rule": [
                            "Reference",
                            [
                              "Resolved",
                              {
                                "source_type_decl": "type t = Field.t",
                                "ref_rule": [
                                  "Reference",
                                  [
                                    "Resolved",
                                    {
                                      "source_type_decl": "Tick.Field.t",
                                      "ref_rule": [ "String" ]
                                    }
                                  ]
                                ]
                              }
                            ]
                          ]
                        }
                      ]
                    ]
                  }
                ]
              ]
            }
          ]
        ],
        [
          "List",
          [
            "Reference",
            [
              "Resolved",
              {
                "source_type_decl": "type t = { version: int ; t: typ }",
                "ref_rule": [
                  "Record",
                  [
                    { "field_name": "version", "field_rule": [ "Int" ] },
                    {
                      "field_name": "t",
                      "field_rule": [
                        "Reference",
                        [
                          "Resolved",
                          {
                            "source_type_decl": "type typ = t",
                            "ref_rule": [
                              "Reference",
                              [
                                "Resolved",
                                {
                                  "source_type_decl": "type t = Field.t",
                                  "ref_rule": [
                                    "Reference",
                                    [
                                      "Resolved",
                                      {
                                        "source_type_decl": "Tick.Field.t",
                                        "ref_rule": [ "String" ]
                                      }
                                    ]
                                  ]
                                }
                              ]
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                ]
              }
            ]
          ]
        ]
      ]
    ]
  ]
}
"#;

    #[test]
    fn test_layout() {
        let layout: Layout = serde_json::from_str(TEST_LAYOUT).unwrap();
        let mut iter = layout.bin_prot_rule.into_branching_iter();
        while let Ok(Some(v)) = iter.next() {
            println!("{:?}\n", v);
        }
    }

    const TEST_LAYOUT_SUM: &str = r#"
{
  "layout_loc": "<unknown>",
  "version_opt": null,
  "type_decl": "<unknown>",
  "bin_io_derived": true,
  "bin_prot_rule":
  [
    "Sum",
    [
      {
        "ctor_name": "Sca1lar_challenge",
        "index": 0,
        "ctor_args": [
          [
            "Reference",
            [
              "Resolved",
              {
                "source_type_decl": "type 'a t = { version: int ; t: 'a typ }",
                "bin_io_derived": false,
                "ref_rule": [
                  "Record",
                  [
                    {
                      "field_name": "version",
                      "field_rule": [
                        "Int"
                      ]
                    },
                    {
                      "field_name": "t",
                      "field_rule": [
                        "Reference",
                        [
                          "Resolved",
                          {
                            "source_type_decl": "type 'a typ = 'a t",
                            "bin_io_derived": false,
                            "ref_rule": [
                              "Reference",
                              [
                                "Resolved",
                                {
                                  "source_type_decl": "'a t = ('a, Nat.N2.n) vec",
                                  "bin_io_derived": false,
                                  "ref_rule": [
                                    "Reference",
                                    [
                                      "Resolved",
                                      {
                                        "source_type_decl": "type t = { version: int ; t: typ }",
                                        "bin_io_derived": false,
                                        "ref_rule": [
                                          "Record",
                                          [
                                            {
                                              "field_name": "version",
                                              "field_rule": [
                                                "Int"
                                              ]
                                            },
                                            {
                                              "field_name": "t",
                                              "field_rule": [
                                                "Reference",
                                                [
                                                  "Resolved",
                                                  {
                                                    "source_type_decl": "type typ = t",
                                                    "bin_io_derived": false,
                                                    "ref_rule": [
                                                      "Reference",
                                                      [
                                                        "Resolved",
                                                        {
                                                          "source_type_decl": "Core_kernel.Int64.t",
                                                          "bin_io_derived": true,
                                                          "ref_rule": [
                                                            "Int64"
                                                          ]
                                                        }
                                                      ]
                                                    ]
                                                  }
                                                ]
                                              ]
                                            }
                                          ]
                                        ]
                                      }
                                    ]
                                  ]
                                }
                              ]
                            ]
                          }
                        ]
                      ]
                    }
                  ]
                ]
              }
            ]
          ]
        ]
      }
    ]
  ]
}"#;

    #[test]
    fn test_layout_sum() {
        let layout: Layout = serde_json::from_str(TEST_LAYOUT_SUM).unwrap();
        let mut iter = layout.bin_prot_rule.into_branching_iter();
        // Test by taking the 0th branch at each branch node. Test is considered as pass
        // if no error
        loop {
            match iter.next() {
                Ok(Some(v)) => {
                    if let BinProtRule::Sum(_) = v {
                        // if its a sum type take the first variant in each case
                        iter.branch(0).expect("Invalid branch index");
                    }
                    println!("{:?}\n", v);
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
    }
}
