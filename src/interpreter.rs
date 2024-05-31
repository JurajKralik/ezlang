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
            ASTNode::Boolean(value) => Token::Boolean(*value),
            ASTNode::Identifier(name) => {
                match self.variables.get(name) {
                    Some(token) => {
                        match token {
                            Token::Number(value) => Token::Number(value.clone()),
                            Token::Boolean(value) => Token::Boolean(value.clone()),
                            _ => panic!("Error i001: Unexpected token: {:?}", token),
                        }
                    }
                    None => panic!("Error i002: Variable not found: {}", name),
                }
            }
            ASTNode::BinaryOperation { left, operator, right } => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                match operator {
                    Token::Plus => {
                        match (left_val, right_val) {
                            (Token::Number(left_num), Token::Number(right_num)) => Token::Number(left_num + right_num),
                            (Token::Number(left_num), Token::Boolean(right_bool)) => Token::Boolean(self.num_to_bool(left_num) || right_bool),
                            (Token::Boolean(left_bool), Token::Number(right_num)) => Token::Boolean(left_bool || self.num_to_bool(right_num)),
                            (Token::Boolean(left_bool), Token::Boolean(right_bool)) => Token::Boolean(left_bool || right_bool),
                            (Token::String(left_str), Token::String(right_str)) => Token::String(format!("{}{}", left_str, right_str)),
                            (Token::String(left_str), Token::Number(right_num)) => Token::String(format!("{}{}", left_str, right_num)), 
                            _ => panic!("Error i003: Unexpected values"),
                        }
                    }
                    Token::Minus => {
                        match (left_val, right_val) {
                            (Token::Number(left_num), Token::Number(right_num)) => Token::Number(left_num - right_num),
                            (Token::Boolean(left_bool), Token::Boolean(right_bool)) => Token::Boolean(left_bool && !right_bool),
                            _ => panic!("Error i003: Unexpected values"),
                            
                        }
                    }
                    Token::Asterisk => {
                        match (left_val, right_val) {
                            (Token::Number(left_num), Token::Number(right_num)) => Token::Number(left_num * right_num),
                            (Token::Boolean(left_bool), Token::Boolean(right_bool)) => Token::Boolean(left_bool && right_bool),
                            _ => panic!("Error i003: Unexpected values"),
                        }
                    }
                    Token::Slash => {
                        if let (Token::Number(left_num), Token::Number(right_num)) = (left_val, right_val) {
                            Token::Number(left_num / right_num)
                        } else {
                            panic!("Error i003: Unexpected values");
                        }
                    }
                    Token::Modulo => {
                        if let (Token::Number(left_num), Token::Number(right_num)) = (left_val, right_val) {
                            Token::Number(left_num % right_num)
                        } else {
                            panic!("Error i003: Unexpected values");
                        }
                    }
                    Token::And => {
                        match (left_val, right_val) {
                            (Token::Boolean(left), Token::Boolean(right)) => Token::Boolean(left && right),
                            (Token::Number(left), Token::Number(right)) => Token::Boolean(self.num_to_bool(left) && self.num_to_bool(right)),
                            (Token::Number(left), Token::Boolean(right)) => Token::Boolean(self.num_to_bool(left) && right),
                            (Token::Boolean(left), Token::Number(right)) => Token::Boolean(left && self.num_to_bool(right)),
                            _ => panic!("Error i003: Unexpected values"),
                        }
                    }
                    Token::Or => {
                        match (left_val, right_val) {
                            (Token::Boolean(left), Token::Boolean(right)) => Token::Boolean(left || right),
                            (Token::Number(left), Token::Number(right)) => Token::Boolean(self.num_to_bool(left) || self.num_to_bool(right)),
                            (Token::Number(left), Token::Boolean(right)) => Token::Boolean(self.num_to_bool(left) || right),
                            (Token::Boolean(left), Token::Number(right)) => Token::Boolean(left || self.num_to_bool(right)),
                            _ => panic!("Error i003: Unexpected values"),
                        }
                    }
                    _ => panic!("Error i004: Unexpected operator: {:?}", operator),
                }
            }
            ASTNode::BindingOperation { variable, value } => {
                let token_value = self.interpret(value);
                let variable_name = match variable {
                    Token::Identifier(name) => name,
                    _ => panic!("Error i005: Unexpected token: {:?}", variable),
                };
                self.variables.insert(variable_name.clone(), token_value.clone());
                token_value
            }
            ASTNode::LogicalOperation { left, operator, right } => {
                Token::Boolean(match operator {
                    Token::And => {
                        let left_val = self.interpret(left);
                        let right_val = self.interpret(right);
                        match (left_val, right_val) {
                            (Token::Boolean(left), Token::Boolean(right)) => left && right,
                            (Token::Number(left), Token::Number(right)) => self.num_to_bool(left) && self.num_to_bool(right),
                            (Token::Number(left), Token::Boolean(right)) => self.num_to_bool(left) && right,
                            (Token::Boolean(left), Token::Number(right)) => left && self.num_to_bool(right),
                            _ => panic!("Error i006: Unexpected values"),
                        }
                    }
                    Token::Or => {
                        let left_val = self.interpret(left);
                        let right_val = self.interpret(right);
                        match (left_val, right_val) {
                            (Token::Boolean(left), Token::Boolean(right)) => left || right,
                            (Token::Number(left), Token::Number(right)) => self.num_to_bool(left) || self.num_to_bool(right),
                            (Token::Number(left), Token::Boolean(right)) => self.num_to_bool(left) || right,
                            (Token::Boolean(left), Token::Number(right)) => left || self.num_to_bool(right),
                            _ => panic!("Error i006: Unexpected values"),
                        }
                    }
                    Token::Not => {
                        let right_val = self.interpret(right);
                        match right_val {
                            Token::Boolean(right) => !right,
                            _ => panic!("Error i006: Unexpected values"),
                        }
                    }
                    _ => panic!("Error i007: Unexpected operator: {:?}", operator),
                })
            }
            ASTNode::ConditionalOperation { condition, true_branch, false_branch } => {
                let condition_val = self.interpret(condition);
                match condition_val {
                    Token::Boolean(true) => self.interpret(true_branch),
                    Token::Boolean(false) => self.interpret(false_branch),
                    _ => panic!("Error i008: Unexpected value: {:?}", condition_val),
                }
            }
        }
    }

    fn num_to_bool(&self, num: i64) -> bool {
        num != 0
    }
}
