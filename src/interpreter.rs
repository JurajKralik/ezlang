use std::collections::HashMap;
use crate::parser::*;
use crate::tokenizer::*;


pub struct Interpreter{
    variables: HashMap<String, Token>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { variables: HashMap::new() }
    }

    pub fn interpret(&self, node: &ASTNode) -> i64 {
        match node {
            ASTNode::Number(value) => *value,
            ASTNode::Identifier(name) => {
                match self.variables.get(name) {
                    Some(token) => {
                        match token {
                            Token::Number(value) => value.parse::<i64>().unwrap(),
                            _ => panic!("Unexpected token: {:?}", token),
                        }
                    }
                    None => panic!("Variable not found: {}", name),
                }
            }
            ASTNode::BinaryOperation { left, operator, right } => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                match operator {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Asterisk => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    Token::Modulo => left_val % right_val,
                    _ => panic!("Unexpected operator: {:?}", operator),
                }
            }
        }
    }
}
