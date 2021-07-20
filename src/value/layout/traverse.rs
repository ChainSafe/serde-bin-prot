use crate::value::layout::{BinProtRule, Summand, RuleRef};

/// Implements a depth first search of the type tree
/// defined by a BinProtRule
pub struct BinProtRuleIterator {
    stack: Vec<BinProtRule>, // regular stack to implement the DFS
    // Tree nodes can branch (only one child should be followed) rather than require traversal of all children
    // If that is the case the parent should add the children to the branch field and the next path will be taken from here rather than the stack
    branch: Option<Vec<Summand>>, 
}

/// An iterator where the next item may require specifying a branch to take
/// At some points in its iteration the iterator will require a call to branch to select which
/// path to take before continuing
trait BranchingIterator {
    type Item;
    type Error;
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error>;
    fn branch(&mut self, branch: usize) -> Result<(), Self::Error>;
}

impl BranchingIterator for BinProtRuleIterator {
    type Item = BinProtRule;
    type Error = String;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
    	if self.branch.is_some() { return Err("Must call branch to proceed".to_string()) }

    	let top = self.stack.pop();
    	let r = top.clone();
        match top {
            Some(rule) => {
            	match rule {
            		BinProtRule::Option(r) | BinProtRule::List(r) => {
            			self.stack.push(*r);
            		},
            		BinProtRule::Record(mut rules) => {
            			self.stack.extend(rules.drain(0..).map(|field| field.field_rule ).rev());
            		},
					BinProtRule::Tuple(mut rules) => {
						self.stack.extend(rules.drain(0..).rev());
					},
					BinProtRule::Sum(summands) => {
		            	// don't add to the stack. Add to the branch field instead
		            	// this must be resolved by calling `branch` before the iterator can continue
		            	self.branch = Some(summands.to_vec());
		            },
		            BinProtRule::Reference(rule_ref) => {
		            	match rule_ref {
		            		RuleRef::Unresolved(_payload) => {
		            			unimplemented!();
		            		},
		            		RuleRef::Resolved(payload) => {
		            			self.stack.push(*payload.ref_rule);
		            		}
		            	}
		            }
		            BinProtRule::String | BinProtRule::Int => {  } // do nothing for these
					r => panic!("unimplemented: {:?}", r)
            	};
            }
            None => return Ok(None), // end of traversal
        };
        Ok(r)
    }

	fn branch(&mut self, branch: usize) -> Result<(), Self::Error> {
		if let Some(mut summands) = self.branch.take() {
        	// check this is the right way around...
        	let s = summands.get_mut(branch).ok_or("Invalid branch".to_string())?;
        	self.stack.extend(s.ctor_args.drain(0..));
        	Ok(())
		} else {
			Err("Cannot branch at this location in the tree".to_string())
		}
	}

}

// Consumes the rule and produces a branching iterator
impl BinProtRule {
    fn into_iter(self) -> BinProtRuleIterator {
        BinProtRuleIterator {
            stack: vec![self],
            branch: None
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
		let mut iter = layout.bin_prot_rule.into_iter();
		while let Ok(Some(v)) = iter.next() {
			println!("{:?}\n", v);
		}
	}
}