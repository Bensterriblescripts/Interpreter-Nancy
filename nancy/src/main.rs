use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use regex::{Error, Regex};
use std::time::{Instant};

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
    let start = Instant::now();
    let path = "C:\\Repositories\\Interpreter-Nancy\\nancy\\test.na";
    let _newline = "\n";
    let text;

    // File Specifics
    let mut variables: Vec<Variable> = Vec::new();
    let mut conditions: Vec<Condition> = Vec::new();
    let mut language = "rust";
    let mut programtype: i8;

    // Compile Regex - Blocks
    let re_program = Regex::new(r"new\s*(?<programtype>\w+)\s*-\s*(?<programname>\w+)")?;

    // Compile Regex - Single Word Types
    // let re_string = Regex::new(r#"\s*"(?<string>.*)""#)?;
    // let re_variable = Regex::new(r"\s*(?<varible>\w+)")?;
    // let re_bool = Regex::new(r"\s*(?<bool>true|false)")?;
    // let re_float = Regex::new(r"\s*(?<float>(?:0|[1-9]\d*)\.\d+)")?;
    // let re_int = Regex::new(r"\s*(?<int>\d+)")?;

    // Compile Regex - Variable Declarations
    let re_assignvar = Regex::new(r"(?<variable>\w+)\s*(equals|=|is)\s*(?<value>\w+)\s*(\d+|\d+.\d+|true|false|.*)")?; // have (assigning a variable)
    let re_allocatearr = Regex::new(r"contain|store|contains|stores")?; // contain (assigning an array)
    // Compile Regex - Assignment Types
    // let re_assignbool = Regex::new(r"(?i)\s*(?<name>\w+)\s*is\s*(?<bool>true|false)")?; // Any word equal to true or false
    // let re_assignint = Regex::new(r"\s*(?<name>\w+)\s*is\s*(?<int>\d+)$")?; // Any number (excludes digits)
    // let re_assignfloat = Regex::new(r"\s*(?<name>\w+)\s*is\s*(?<float>(?:0|[1-9]\d*)\.\d +)")?; // Any number with digits
    // let re_assignstring = Regex::new(r#"\s*(?<name>\w+)\s*is\s*"(?<string>.*)""#)?; // Any collection of characters

    // Compile Regex - Conditions
    let re_if = Regex::new(r"(?i)\s.*(if|else if|is|or is)\s*(?<left>\d+|\d+.\d+|true|false|\w+)\s*(?<operator>(is|==|===|!=|!==|greater than|less than|greater than or equal to|less than or equal to|>=|>|<=|<))\s*(?<right>\d+|\d+.\d+|true|false|.*)")?; // equals (conditional match)

    // Compile Regex - Loops
    let re_loopwhile = Regex::new(r"while\s*(?<left>\d+|\d+.\d+|true|false|.*)\s*(?<operator>>|>=|<|<=|=|==|===|is|is not)\s*(?<right>\d+|\d+.\d+|true|false|.*)")?;
    let re_looprun = Regex::new(r"run\s*(?<iter>\d+)\s*times")?; // run function 15 times

    // TODO: Convert document to utf8

    // Extract Text
    match open_file(path) {
        Ok(contents) => {
            text = contents;
            println!("\n{}\n", text);
        },
        Err(error) => panic!("Error opening file: {}\n", error),
    }

    // TODO: Change all stored characters to lowercase

    // Iterate Through
    let mut linenumber: i32 = 0;
    for line in text.lines() {
        linenumber += 1;

        // First Character
        for first in line.chars() {
            if first == ' ' { continue; }
            if first == '/' { // Comment
                break;
            }
            if first == 'n' { // Program
                if let Some(caps) = re_program.captures(line) {
                    programtype = new_program(&caps, linenumber);
                    if programtype == 0 {
                        let errormessage = incorrect_program(&caps["programtype"]);
                        println!("Line {}: {}",linenumber, errormessage);
                        exit(0);
                    }
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
                    new_whileloop(caps, linenumber);
                    break;
                }
                if let Some(caps) = re_looprun.captures(line) {
                    new_forloop(caps, linenumber);
                    break;
                }
            }
            break;
        }

        // Check variables
        if let Some(caps) = re_assignvar.captures(line) {
            new_variable(caps, linenumber);
        }

        // TODO: Unrecognised Line Entry - Record/Error This
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

    let duration = start.elapsed();
    println!("\nTime Taken: {:?}", duration);

    Ok(())
}
fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn new_program(line: &regex::Captures, linenumber: i32) -> i8 {
    println!("New program on line: {}\nProgram Type: {}, Program Name: {}\n", linenumber, &line["programtype"], &line["programname"]);
    if &line["programtype"] == "console" {
        return 1;
    }
    return 0;
}

// Parsing
fn new_condition(line: regex::Captures, linenumber: i32) {
    println!("Found a condition on line: {}\nLeft Variable: {}, Right Variable: {}, Operator: {}\n", linenumber, &line["left"], &line["right"], &line["operator"]);
}
fn new_whileloop(line: regex::Captures, linenumber: i32) {
    println!("Found a while loop on line {}\nLeft: {}, Operator: {}, Right: {}", linenumber, &line["left"], &line["operator"], &line["right"]);
}
fn new_forloop(line: regex::Captures, linenumber: i32) {
    println!("Found a for loop on line {}\nIterates {} times.", linenumber, &line["iter"]);
}
fn new_variable(line: regex::Captures, linenumber: i32) {
    println!("Found a variable assignment on line {}\nName: {}, Value: {}\n", linenumber, &line["variable"], &line["value"]);
}

// Error Reporting
fn incorrect_program(programtype: &str) -> &'static str {
    let re_console = Regex::new(r"c?onsol?e?").unwrap();
    if re_console.is_match(programtype) {
        return "Incorrect Program Type.\n - Did you mean console?\n";
    }
    "Unknown Program Type\n - Options available are: console\n"
}

