// src/rdl/parser.rs

use pest::Parser;
use pest_derive::Parser;
use super::models::{Robot, Part, Attribute, AttachmentPoint, Position};

#[derive(Parser)]
#[grammar = "rdl/rdl.pest"]
struct RdlParser;

fn parse_position(position_pair: pest::iterators::Pair<Rule>) -> Position {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for field in position_pair.into_inner() {
        match field.as_rule() {
            Rule::x_field => x = field.into_inner().next().unwrap().as_str().parse::<f32>().unwrap(),
            Rule::y_field => y = field.into_inner().next().unwrap().as_str().parse::<f32>().unwrap(),
            Rule::z_field => z = field.into_inner().next().unwrap().as_str().parse::<f32>().unwrap(),
            _ => unreachable!(),
        }
    }

    Position { x, y, z }
}

fn parse_attachment_points(pair: pest::iterators::Pair<Rule>) -> Vec<AttachmentPoint> {
    let mut attachment_points = Vec::new();

    for attachment_point_pair in pair.into_inner() {
        let mut attachment_point_name = String::new();
        let mut position = Position { x: 0.0, y: 0.0, z: 0.0 };

        for attachment_inner_pair in attachment_point_pair.into_inner() {
            match attachment_inner_pair.as_rule() {
                Rule::category => attachment_point_name = attachment_inner_pair.as_str().trim_matches('"').to_string(),
                Rule::position => position = parse_position(attachment_inner_pair),
                _ => unreachable!(),
            }
        }

        attachment_points.push(AttachmentPoint { name: attachment_point_name, position });
    }

    attachment_points
}

fn parse_part(part_pair: pest::iterators::Pair<Rule>) -> Part {
    let mut part_category = String::new();
    let mut attributes = Vec::new();
    let mut attachment_points = Vec::new();

    for part_inner_pair in part_pair.into_inner() {
        match part_inner_pair.as_rule() {
            Rule::category => part_category = part_inner_pair.as_str().trim_matches('"').to_string(),
            Rule::attribute => {
                let mut key = String::new();
                let mut value = String::new();
                for attr_pair in part_inner_pair.into_inner() {
                    match attr_pair.as_rule() {
                        Rule::key => key = attr_pair.as_str().to_string(),
                        Rule::value => value = attr_pair.as_str().trim_matches('"').to_string(),
                        _ => unreachable!(),
                    }
                }
                attributes.push(Attribute { key, value });
            }
            Rule::attachment_points => attachment_points = parse_attachment_points(part_inner_pair),
            _ => unreachable!(),
        }
    }

    Part { category: part_category, attributes, attachment_points }
}

fn parse_robot(pair: pest::iterators::Pair<Rule>) -> Robot {
    let mut robot_category = String::new();
    let mut parts = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::category => robot_category = inner_pair.as_str().trim_matches('"').to_string(),
            Rule::parts => {
                for part_pair in inner_pair.into_inner() {
                    parts.push(parse_part(part_pair));
                }
            }
            _ => unreachable!(),
        }
    }

    Robot { category: robot_category, parts }
}

pub fn parse_rdl(input: &str) -> Result<Robot, pest::error::Error<Rule>> {
    let pairs = RdlParser::parse(Rule::file, input)?;
    println!("Parsed pairs: {:?}", pairs);  // Debug statement

    let first_span = pairs.clone().next().unwrap().as_span();
    let mut robot: Option<Robot> = None;

    for pair in pairs {
        println!("Top level pair: {:?}", pair.as_rule());  // Debug statement
        if pair.as_rule() == Rule::robot {
            robot = Some(parse_robot(pair));
        }
    }

    match robot {
        Some(r) => {
            println!("Robot: {:?}", r);  // Debug statement
            Ok(r)
        }
        None => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: String::from("Robot definition not found") },
            first_span,
        )),
    }
}
