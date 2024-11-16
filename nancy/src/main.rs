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

    // Compile Regex - Single Word Types
    let re_string = Regex::new(r#"\s*"(?<string>.*)""#)?;
    let re_variable = Regex::new(r"\s*(?<varible>\w+)")?;
    let re_bool = Regex::new(r"\s*(?<bool>true|false)")?;
    let re_float = Regex::new(r"\s*(?<float>(?:0|[1-9]\d*)\.\d+)")?;
    let re_int = Regex::new(r"\s*(?<int>\d+)")?;

    // Compile Regex - Variable Declarations
    let re_assignvar = Regex::new(r"let|have")?; // have (assigning a variable)
    let re_allocatearr = Regex::new(r"contain|store|contains|stores")?; // contain (assigning an array)
    // Compile Regex - Assignment Types
    let re_assignbool = Regex::new(r"(?i)\s*(?<name>\w+)\s*is\s*(?<bool>true|false)")?; // Any word equal to true or false
    let re_assignint = Regex::new(r"\s*(?<name>\w+)\s*is\s*(?<int>\d+)$")?; // Any number (excludes digits)
    let re_assignfloat = Regex::new(r"\s*(?<name>\w+)\s*is\s*(?<float>(?:0|[1-9]\d*)\.\d+)")?; // Any number with digits
    let re_assignstring = Regex::new(r#"\s*(?<name>\w+)\s*is\s*"(?<string>.*)""#)?; // Any collection of characters

    // Compile Regex - Conditions
    let re_ifequals = Regex::new(r"(?i)\s.*(if|else if)\s*(?<left>\d+|\d+.\d+|true|false|.*)\s*(is|equals)\s*(?<right>\d+|\d+.\d+|true|false|.*)")?; // equals (conditional match)
    let re_ifnotequals = Regex::new(r"does not equal")?; // does not equal (conditional negative match)

    // Compile Regex - Loops
    let re_loopwhilecond = Regex::new(r"\s*while(?<left>\d+|\d+.\d+|true|false|.*)\s*(?<operator>>|>=|<|<=|=|==|===)\s*(?<right>\d+|\d+.\d+|true|false|.*)");
    let re_loopwhiletrue = Regex::new(r"\s*while(?<var>\w+)");
    let re_loopint = Regex::new(r"\s*while(?<iter>\d+)")?;
    let re_loopintfunction = Regex::new(r"\s*run\s*(?<function>\w+)\s*(?<iter>\d+)\s*times"); // run function 15 times

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

        // 0 - Comment, 1 - Variable, 2 - Condition, 3 - Loop, 4 - Function
        let linetype: i8;

        // First Character
        for first in line.chars() {
            if first == ' ' { continue; }
            if first == '/' { // Comment
                linetype = 0;
                break;
            }
            if first == 'i' { // Condition
                if let Some(caps) = re_ifequals.captures(line) {
                    linetype = 2;
                    break;
                }
                else if let Some(caps) = re_ifnotequals.captures(line) {
                    linetype = 2;
                    break;
                }
                else {
                    // if is_variable(first) {
                    //     linetype = 1;
                    //     break;
                    // }
                }
            }
            if first == 'l' || first == 'h' { // Variable (let/have)

            }
            if first == 'w' || first == 'r' { // Loop (while/run)

            }
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
    //         else if let Some(caps) = re_assignint.captures(line) {
    //             println!("Found an integer declaration on line {}", linenumber);
    //             variables.push(Variable {
    //                 name: String::from(&caps["name"]),
    //                 var_type: String::from("Integer"),
    //                 scope: 0,
    //                 data: Type::Integer(caps["int"].parse().unwrap()),
    //             });
    //         }
    //         else if let Some(caps) = re_assignfloat.captures(line) {
    //             println!("Found a float declaration on line {}", linenumber);
    //             variables.push(Variable {
    //                 name: String::from(&caps["name"]),
    //                 var_type: String::from("Float"),
    //                 scope: 0,
    //                 data: Type::Float(caps["float"].parse().unwrap()),
    //             });
    //         }
    //         else if let Some(caps) = re_assignstring.captures(line) {
    //             println!("Found a string declaration on line {}", linenumber);
    //             variables.push(Variable {
    //                 name: String::from(&caps["name"]),
    //                 var_type: String::from("String"),
    //                 scope: 0,
    //                 data: Type::String(String::from(&caps["string"])),
    //             });
    //         }
    //     }
    //     else if re_are.is_match(line) {
    //         println!("Found an array declaration on line {}", linenumber);
    //     }
    //
    //
    //     /* Condition */
    //
    //     // Equals
    //     if let Some(caps) = re_ifequals.captures(line).or_else(|| re_ifequals.captures(line)) {
    //         println!("Identified condition on line {}\nLeft variable, {}, Right variable: {}", linenumber, &caps["left"], &caps["right"]);
    //
    //         // 0 - Var, 1 - Bool, 2 - Int, 3 - Float, 4 - String
    //         let mut lefttype: i8 = 0;
    //         let mut righttype: i8 = 0;
    //
    //         // Left Argument
    //         if let Some(caps) = re_string.captures(&caps["left"]) {
    //             println!("Left assignment is a string");
    //             lefttype = 4;
    //         }
    //         else if let Some(caps) = re_variable.captures(&caps["left"]) {
    //             println!("Left assignment is a variable name");
    //             // Iterate through the existing variables to check if it exists
    //         }
    //         else if let Some(caps) = re_bool.captures(&caps["left"]) {
    //             println!("Left assignment is a bool");
    //             lefttype = 1
    //         }
    //
    //         // Right Argument
    //         if let Some(caps) = re_string.captures(&caps["right"]) {
    //             println!("Right assignment is a string");
    //             righttype = 4;
    //         }
    //         else if let Some(caps) = re_variable.captures(&caps["right"]) {
    //             println!("Right assignment is a variable name");
    //             // Iterate through the existing variables to check if it exists
    //         }
    //         else if let Some(caps) = re_bool.captures(&caps["right"]) {
    //             println!("Right assignment is a bool");
    //             righttype = 1
    //         }
    //
    //         // Only String-Int/Floats Can Mismatch
    //         if lefttype != righttype {
    //
    //         }
    //
    //
    //
    //         let leftvar: &Variable;
    //         let rightvar: &Variable;
    //         //
    //         // // Left Variable Exists
    //         // if let Some(index) = variables.iter().position(|v| v.name.contains(caps["left"].parse().unwrap())) {
    //         //     leftvar = &variables[index];
    //         //     println!("The left variable exists")
    //         // }
    //         // // Right Variable Exists
    //         // if let Some(index) = variables.iter().position(|v| v.name.contains(caps["right"].parse().unwrap())) {
    //         //     rightvar = &variables[index];
    //         //     println!("The right variable exists");
    //     }
    //
    //         // conditions.push(Condition {
    //         //     position: 0,
    //         //     left: caps["left"].parse().unwrap(),
    //         //     right: ,
    //         //     condition_type:
    //         // });
    //     // Not Equal
    //     else if let Some(caps) = re_ifequals.captures(line) {
    //         println!("Found an if-equals on line {}", linenumber);
    //     }
    //
    //     linenumber += 1;
    // }
    //
    // println!("\n");
    // for variable in variables {
    //     println!("Variable: {:?}", variable);
    }

    Ok(())
}
fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn handle_if() {

}
fn handle_loop() {

}

