pub mod tokenizer;
pub mod parser;
pub mod interpreter;

use crate::tokenizer::*;
use crate::parser::*;
use crate::interpreter::*;

fn split_to_lines(program: &str) -> Vec<&str> {
    program.split('\n').collect()
}

fn main() {
    let program = "x = 3 + 6 / (3 - 1) % 2";
    let mut interpreter = Interpreter::new();

    for line in split_to_lines(program) {
        let tokenizer = Tokenizer::new(line);
        println!("{:?}", tokenizer);
        let mut parser = Parser::new(tokenizer);
        let ast = parser.parse();
        println!("{:#?}", ast);
    
        let result = interpreter.interpret(&ast);
        println!("Result: {}", result);
    }
}