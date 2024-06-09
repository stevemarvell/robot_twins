// src/rdl/parser.rs

use pest::Parser;
use pest_derive::Parser;
use super::models::{Robot, Part, Attribute};

#[derive(Parser)]
#[grammar = "rdl/rdl.pest"]
struct RdlParser;

pub fn parse_rdl(input: &str) -> Result<Robot, pest::error::Error<Rule>> {
    let pairs = RdlParser::parse(Rule::file, input)?;

    let mut robot_category = String::new();
    let mut parts = Vec::new();
    let mut robot_span = None;

    for pair in pairs {
        if pair.as_rule() == Rule::robot {
            robot_span = Some(pair.as_span());
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::category {
                    robot_category = inner_pair.as_str().trim_matches('"').to_string();
                } else if inner_pair.as_rule() == Rule::parts {
                    for part_pair in inner_pair.into_inner() {
                        let mut part_category = String::new();
                        let mut attributes = Vec::new();
                        for part_inner_pair in part_pair.into_inner() {
                            if part_inner_pair.as_rule() == Rule::category {
                                part_category = part_inner_pair.as_str().trim_matches('"').to_string();
                            } else if part_inner_pair.as_rule() == Rule::attribute {
                                let mut key = String::new();
                                let mut value = String::new();
                                for attr_pair in part_inner_pair.into_inner() {
                                    match attr_pair.as_rule() {
                                        Rule::key => key = attr_pair.as_str().to_string(),
                                        Rule::value => value = attr_pair.as_str().trim_matches('"').to_string(),
                                        _ => {}
                                    }
                                }
                                attributes.push(Attribute { key, value });
                            }
                        }
                        parts.push(Part { category: part_category, attributes });
                    }
                }
            }
        }
    }

    let robot = Robot { category: robot_category, parts };
    println!("Robot: {:?}", robot);  // Debug statement

    Ok(robot)
}
