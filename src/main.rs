pub mod tokenizer;
pub mod parser;
pub mod interpreter;

use crate::tokenizer::*;
use crate::parser::*;
use crate::interpreter::*;


fn main() {
    let program = "3 + 6 / (3 - 1) * 2";
    let tokenizer = Tokenizer::new(program);
    let mut parser = Parser::new(tokenizer);
    let ast = parser.parse();
    println!("{:#?}", ast);

    let interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast);
    println!("Result: {}", result);
}