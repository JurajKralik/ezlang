pub mod interpreter;
pub mod parser;
pub mod tokenizer;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use crate::interpreter::*;
use crate::parser::*;
use crate::tokenizer::*;

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn split_to_lines(program: &str) -> Vec<&str> {
    program.split('\n').collect()
}

fn main() {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: Provide a file path as an argument");
    }
    let file_path = &args[1];
    if !file_path.ends_with(".ez") {
        panic!("Error: The file does not have an .ez extension");
    }
    let mut debugger = false;
    if args.len() == 3 {
        debugger = &args[2] == "debug";
    }

    match read_file_to_string(file_path) {
        Ok(content) => {
            let mut interpreter = Interpreter::new();

            for line in split_to_lines(content.as_str()) {
                if debugger {
                    println!("__________________");
                    println!("Line: {}", line);
                }
                let tokenizer = Tokenizer::new(line);
                if debugger {
                    println!("{:?}", tokenizer);
                }
                let mut parser = Parser::new(tokenizer);
                let ast = parser.parse();
                if debugger {
                    println!("AST: {:#?}", ast);
                    println!("{:?}", interpreter);
                }
                let result = interpreter.interpret_line(&ast);
                if debugger {
                    println!("Result: {:?}", result);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
