use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let path = "C:\\Repositories\\Interpreter-Nancy\\nancy\\test.na";
    let _newline = "\n";
    let text;

    // Store Created Variables
    let strings: Vec<&str> = Vec::new();

    // Compile Regex - Syntax
    let re_equals = Regex::new(r"=").unwrap(); // =
    let re_doubleequals = Regex::new(r"==").unwrap(); // ==

    // Compile Regex - Types
    let re_int = Regex::new(r"(?m)^[0-9]$").unwrap(); // Any number (excludes digits)
    let re_float = Regex::new(r"(?m)^[0-9].[0-9]$").unwrap(); // Any number with digits

    // Extract Text
    match open_file(path) {
        Ok(contents) => {
            text = contents;
            println!("{}\n", text);
        },
        Err(error) => panic!("Error opening file: {}", error),
    }

    // Iterate Through
    let mut linenumber: i32 = 1;
    for line in text.lines() {

        // Variable Assignments
        if re_equals.is_match(line){
            println!("Found a variable assignment on line {}", linenumber);
        }

        // Condition
        if re_doubleequals.is_match(line){
            println!("Found a conditional on line {}", linenumber);
        }

        linenumber += 1;
    }
}
fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
