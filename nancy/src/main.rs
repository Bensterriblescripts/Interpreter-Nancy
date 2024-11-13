use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path = "C:\\Repositories\\Interpreter-Nancy\\nancy\\test.na";
    let newline = "\n";
    let text;

    // Extract Text
    match open_file(path) {
        Ok(contents) => {
            text = contents;
            println!("{}", text);
        },
        Err(error) => println!("Error opening file: {}", error),
    }
}
fn open_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}