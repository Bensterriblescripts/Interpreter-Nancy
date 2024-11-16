use std::fs::File;
use std::io::prelude::*;
use regex::{Error, Regex};

#[derive(Debug)]
struct Scopes {
    name: String,
    level: i8,
}
#[derive(Debug)]
enum Block {
    File(String),
    Function(String),
    Condition(Type, Type),
    Loop(Type, Type),
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    var_type: String,
    scope: i8,
    data: Type,
}
#[derive(Debug, Clone)]
enum Type {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
enum ConditionTypes {
    Equal(bool),
    NotEqual(bool),
    Contain(bool),
    NotContain(bool),
}
#[derive(Debug, Clone)]
struct Condition {
    position: i8,
    left: Variable,
    right: Variable,
    condition_type: ConditionTypes,
}

fn main() -> Result<(), Error> {
    let path = "C:\\Repositories\\Interpreter-Nancy\\nancy\\test.na";
    let _newline = "\n";
    let text;

    // Store Created Variables
    let mut variables: Vec<Variable> = Vec::new();
    let mut conditions: Vec<Condition> = Vec::new();

    // Compile Regex - Blocks
    let re_program = Regex::new(r"new\s*(?<program>\w+)\s*-\s*(?<programtype>\w+)")?;

    // Compile Regex - Single Word Types
    let re_string = Regex::new(r#"\s*"(?<string>.*)""#)?;
    let re_variable = Regex::new(r"\s*(?<varible>\w+)")?;
    let re_bool = Regex::new(r"\s*(?<bool>true|false)")?;
    let re_float = Regex::new(r"\s*(?<float>(?:0|[1-9]\d*)\.\d+)")?;
    let re_int = Regex::new(r"\s*(?<int>\d+)")?;

    // Compile Regex - Variable Declarations
    let re_assignvar = Regex::new(r"(?<variable>\w+)\s*(equals|=|==|is)(?<value>\w+)\s*(\d+|\d+.\d+|true|false|.*)")?; // have (assigning a variable)
    let re_allocatearr = Regex::new(r"contain|store|contains|stores")?; // contain (assigning an array)
    // Compile Regex - Assignment Types
    let re_assignbool = Regex::new(r"(?i)\s*(?<name>\w+)\s*is\s*(?<bool>true|false)")?; // Any word equal to true or false
    let re_assignint = Regex::new(r"\s*(?<name>\w+)\s*is\s*(?<int>\d+)$")?; // Any number (excludes digits)
    let re_assignfloat = Regex::new(r"\s*(?<name>\w+)\s*is\s*(?<float>(?:0|[1-9]\d*)\.\d +)")?; // Any number with digits
    let re_assignstring = Regex::new(r#"\s*(?<name>\w+)\s*is\s*"(?<string>.*)""#)?; // Any collection of characters

    // Compile Regex - Conditions
    let re_if = Regex::new(r"(?i)\s.*(if|else if|is|or is)\s*(?<left>\d+|\d+.\d+|true|false|\w+)\s*(?<operator>(is|equals|==|===|!=|!==|greater than|less than|greater than or equal to|less than or equal to|>=|>|<=|<))\s*(?<right>\d+|\d+.\d+|true|false|.*)")?; // equals (conditional match)

    // Compile Regex - Loops
    let re_loopwhile = Regex::new(r"\s*while(?<left>\d+|\d+.\d+|true|false|.*)\s*(?<operator>>|>=|<|<=|=|==|===|is|is not)\s*(?<right>\d+|\d+.\d+|true|false|.*)");
    let re_looprun = Regex::new(r"\s*run\s*(?<function>\w+)\s*(?<iter>\d+)\s*times"); // run function 15 times

    // TODO: Convert document to utf8

    // Extract Text
    match open_file(path) {
        Ok(contents) => {
            text = contents;
            println!("\n{}\n", text);
        },
        Err(error) => panic!("Error opening file: {}", error),
    }

    // TODO: Change all stored characters to lowercase

    // Iterate Through
    let mut linenumber: i32 = 1;
    for line in text.lines() {

        // 0 - Comment, 1 - Variable, 2 - Condition, 3 - Loop, 4 - Function, 10 - Program
        let mut linetype: i8 = -1;

        // First Character
        for first in line.chars() {
            if first == ' ' { continue; }
            if first == '/' { // Comment
                linetype = 0;
                break;
            }
            if first == 'n' { // Program
                if let Some(caps) = re_program.captures(line) {
                    new_program(caps, linenumber);
                    break;
                }
            }
            if first == 'i' { // Condition
                if let Some(caps) = re_if.captures(line) {
                    new_condition(caps, linenumber);
                    break;
                }
            }
            if first == 'w' || first == 'r' { // Loop (while/run) {
                if let Some(caps) = re_loopwhile.captures(line) {
                    new_loop(caps, linenumber);
                }
            }

            // Check variables
        }
        linenumber += 1;

        // End on irrelevant
        if linetype == -1 || linetype == 0 {
            continue;
        }

        /* Variable */
    //
    //     // Variable Assignments
    //     if re_is.is_match(line) {
    //         if let Some(caps) = re_assignbool.captures(line) {
    //             println!("Found a bool declaration on line {}", linenumber);
    //             if caps["bool"].contains("true") {
    //                 variables.push(Variable {
    //                     name: String::from(&caps["name"]),
    //                     var_type: String::from("Boolean"),
    //                     scope: 0,
    //                     data: Type::Boolean(true),
    //                 });
    //             }
    //             else if caps["bool"].contains("false") {
    //                 variables.push(Variable {
    //                     name: String::from(&caps["name"]),
    //                     var_type: String::from("Boolean"),
    //                     scope: 0,
    //                     data: Type::Boolean(false),
    //                 });
    //             }
    //         }
    }

    Ok(())
}
fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn new_program(line: regex::Captures, linenumber: i32) {
    println!("Created a new program on line: {}\nProgram Type: {}, Program Name: {}\n", linenumber, &line["program"], &line["programtype"]);
}
fn new_condition(line: regex::Captures, linenumber: i32) {
    println!("Found a condition on line: {}\nLeft Variable: {}, Right Variable: {}, Operator: {}\n", linenumber, &line["left"], &line["right"], &line["operator"]);
}
fn new_loop(line: regex::Captures, linenumber: i32) {

}

