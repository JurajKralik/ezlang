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

    pub fn interpret(&mut self, node: &ASTNode) -> Token {
        match node {
            ASTNode::Number(value) => Token::Number(*value),
            ASTNode::Identifier(name) => {
                match self.variables.get(name) {
                    Some(token) => {
                        match token {
                            Token::Number(value) => Token::Number(value.clone()),
                            _ => panic!("Error: Unexpected token: {:?}", token),
                        }
                    }
                    None => panic!("Error: Variable not found: {}", name),
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
                    _ => panic!("Error: Unexpected operator: {:?}", operator),
                }
            }
            ASTNode::BindingOperation { variable, value } => {
                let token_value = self.interpret(value);
                let variable_name = match variable {
                    Token::Identifier(name) => name,
                    _ => panic!("Error: Unexpected token: {:?}", variable),
                };
                self.variables.insert(variable_name.clone(), token_value);
                token_value
            }
            ASTNode::LogicalOperation { left, operator, right } => {
                Token::Boolean(match operator {
                    Token::And => {
                        let left_val = self.interpret(left);
                        let right_val = self.interpret(right);
                        match (left_val, right_val) {
                            (Token::Boolean(left), Token::Boolean(right)) => left && right,
                            _ => panic!("Error: Unexpected values: {:?}, {:?}", left_val, right_val),
                        }
                    }
                    Token::Or => {
                        let left_val = self.interpret(left);
                        let right_val = self.interpret(right);
                        match (left_val, right_val) {
                            (Token::Boolean(left), Token::Boolean(right)) => left || right,
                            _ => panic!("Error: Unexpected values: {:?}, {:?}", left_val, right_val),
                        }
                    }
                    _ => panic!("Error: Unexpected operator: {:?}", operator),
                })
            }
        }
    }
}
