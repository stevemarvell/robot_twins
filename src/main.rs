mod rdl;

use std::fs;
use rdl::parse_rdl;

fn main() {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    println!("Current directory: {:?}", current_dir);

    let rdl_content = fs::read_to_string("configs/hello_robot.rdl")
        .expect("Unable to read rdl file");

    match parse_rdl(&rdl_content) {
        Ok(robot) => println!("{:#?}", robot),
        Err(e) => println!("Error parsing RDL: {}", e),
    }
}