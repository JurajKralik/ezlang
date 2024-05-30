pub mod tokenizer;

use crate::tokenizer::*;


fn main() {
    let program = "3 + 6 / (3 - 1)";
    let tokenizer = Tokenizer::new(program);
    println!("{:#?}", tokenizer);
}