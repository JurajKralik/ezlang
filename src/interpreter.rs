use crate::parser::*;
use crate::tokenizer::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter {
    variables: HashMap<String, Token>,
    pass: bool,
    indent_level: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            pass: false,
            indent_level: 0,
        }
    }

    pub fn interpret_line(&mut self, node: &ASTNode) -> Token {
        if self.pass {
            let mut local_indent_level = 0;
            match node {
                ASTNode::Number(_value, indent_level) => local_indent_level = indent_level.clone(),
                ASTNode::Boolean(_value, node_indent_level) => {
                    local_indent_level = node_indent_level.clone()
                }
                ASTNode::Identifier(_name, node_indent_level) => {
                    local_indent_level = node_indent_level.clone()
                }
                ASTNode::BinaryOperation {
                    left: _,
                    operator: _,
                    right: _,
                    indent_level,
                } => local_indent_level = indent_level.clone(),
                ASTNode::BindingOperation {
                    variable: _,
                    value: _,
                    indent_level,
                } => local_indent_level = indent_level.clone(),
                ASTNode::LogicalOperation {
                    left: _,
                    operator: _,
                    right: _,
                    indent_level,
                } => local_indent_level = indent_level.clone(),
                ASTNode::ConditionalOperation {
                    condition: _,
                    indent_level,
                } => local_indent_level = indent_level.clone(),
                ASTNode::AlternativeOperation {
                    condition,
                    indent_level,
                } => {
                    local_indent_level = indent_level.clone();
                    if local_indent_level == self.indent_level {
                        if let Some(elif_condition) = condition {
                            //Elif
                            if self.interpret(elif_condition) == Token::Boolean(true) {
                                self.pass = false;
                                return Token::Boolean(true);
                            } else {
                                self.pass = true;
                                return Token::Boolean(false);
                            }
                        } else {
                            //Else
                            self.pass = false;
                            return Token::Boolean(true)
                        }
                    }
                }
            }
            if local_indent_level > self.indent_level && self.pass {
                return Token::None;
            } else {
                self.pass = false;
                return Token::Boolean(true)
            }
        }
        self.interpret(node)
    }

    pub fn interpret(&mut self, node: &ASTNode) -> Token {
        match node {
            ASTNode::Number(value, _indent_level) => Token::Number(*value),
            ASTNode::Boolean(value, _indent_level) => Token::Boolean(*value),
            ASTNode::Identifier(name, _indent_level) => match self.variables.get(name) {
                Some(token) => match token {
                    Token::Number(value) => Token::Number(value.clone()),
                    Token::Boolean(value) => Token::Boolean(value.clone()),
                    _ => panic!("Error i001: Unexpected token: {:?}", token),
                },
                None => panic!("Error i002: Variable not found: {}", name),
            },
            ASTNode::BinaryOperation {
                left,
                operator,
                right,
                indent_level: _,
            } => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                match operator {
                    Token::Plus => match (left_val, right_val) {
                        (Token::Number(left_num), Token::Number(right_num)) => {
                            Token::Number(left_num + right_num)
                        }
                        (Token::Number(left_num), Token::Boolean(right_bool)) => {
                            Token::Boolean(self.num_to_bool(left_num) || right_bool)
                        }
                        (Token::Boolean(left_bool), Token::Number(right_num)) => {
                            Token::Boolean(left_bool || self.num_to_bool(right_num))
                        }
                        (Token::Boolean(left_bool), Token::Boolean(right_bool)) => {
                            Token::Boolean(left_bool || right_bool)
                        }
                        (Token::String(left_str), Token::String(right_str)) => {
                            Token::String(format!("{}{}", left_str, right_str))
                        }
                        (Token::String(left_str), Token::Number(right_num)) => {
                            Token::String(format!("{}{}", left_str, right_num))
                        }
                        _ => panic!("Error i003: Unexpected values"),
                    },
                    Token::Minus => match (left_val, right_val) {
                        (Token::Number(left_num), Token::Number(right_num)) => {
                            Token::Number(left_num - right_num)
                        }
                        (Token::Boolean(left_bool), Token::Boolean(right_bool)) => {
                            Token::Boolean(left_bool && !right_bool)
                        }
                        _ => panic!("Error i003: Unexpected values"),
                    },
                    Token::Asterisk => match (left_val, right_val) {
                        (Token::Number(left_num), Token::Number(right_num)) => {
                            Token::Number(left_num * right_num)
                        }
                        (Token::Boolean(left_bool), Token::Boolean(right_bool)) => {
                            Token::Boolean(left_bool && right_bool)
                        }
                        _ => panic!("Error i003: Unexpected values"),
                    },
                    Token::Slash => {
                        if let (Token::Number(left_num), Token::Number(right_num)) =
                            (left_val, right_val)
                        {
                            Token::Number(left_num / right_num)
                        } else {
                            panic!("Error i003: Unexpected values");
                        }
                    }
                    Token::Modulo => {
                        if let (Token::Number(left_num), Token::Number(right_num)) =
                            (left_val, right_val)
                        {
                            Token::Number(left_num % right_num)
                        } else {
                            panic!("Error i003: Unexpected values");
                        }
                    }
                    Token::And => match (left_val, right_val) {
                        (Token::Boolean(left), Token::Boolean(right)) => {
                            Token::Boolean(left && right)
                        }
                        (Token::Number(left), Token::Number(right)) => {
                            Token::Boolean(self.num_to_bool(left) && self.num_to_bool(right))
                        }
                        (Token::Number(left), Token::Boolean(right)) => {
                            Token::Boolean(self.num_to_bool(left) && right)
                        }
                        (Token::Boolean(left), Token::Number(right)) => {
                            Token::Boolean(left && self.num_to_bool(right))
                        }
                        _ => panic!("Error i003: Unexpected values"),
                    },
                    Token::Or => match (left_val, right_val) {
                        (Token::Boolean(left), Token::Boolean(right)) => {
                            Token::Boolean(left || right)
                        }
                        (Token::Number(left), Token::Number(right)) => {
                            Token::Boolean(self.num_to_bool(left) || self.num_to_bool(right))
                        }
                        (Token::Number(left), Token::Boolean(right)) => {
                            Token::Boolean(self.num_to_bool(left) || right)
                        }
                        (Token::Boolean(left), Token::Number(right)) => {
                            Token::Boolean(left || self.num_to_bool(right))
                        }
                        _ => panic!("Error i003: Unexpected values"),
                    },
                    _ => panic!("Error i004: Unexpected operator: {:?}", operator),
                }
            }
            ASTNode::BindingOperation {
                variable,
                value,
                indent_level: _,
            } => {
                let token_value = self.interpret(value);
                let variable_name = match variable {
                    Token::Identifier(name) => name,
                    _ => panic!("Error i005: Unexpected token: {:?}", variable),
                };
                self.variables
                    .insert(variable_name.clone(), token_value.clone());
                token_value
            }
            ASTNode::LogicalOperation {
                left,
                operator,
                right,
                indent_level: _,
            } => Token::Boolean(match operator {
                Token::And => {
                    let left_val = self.interpret(left);
                    let right_val = self.interpret(right);
                    match (left_val, right_val) {
                        (Token::Boolean(left), Token::Boolean(right)) => left && right,
                        (Token::Number(left), Token::Number(right)) => {
                            self.num_to_bool(left) && self.num_to_bool(right)
                        }
                        (Token::Number(left), Token::Boolean(right)) => {
                            self.num_to_bool(left) && right
                        }
                        (Token::Boolean(left), Token::Number(right)) => {
                            left && self.num_to_bool(right)
                        }
                        _ => panic!("Error i006: Unexpected values"),
                    }
                }
                Token::Or => {
                    let left_val = self.interpret(left);
                    let right_val = self.interpret(right);
                    match (left_val, right_val) {
                        (Token::Boolean(left), Token::Boolean(right)) => left || right,
                        (Token::Number(left), Token::Number(right)) => {
                            self.num_to_bool(left) || self.num_to_bool(right)
                        }
                        (Token::Number(left), Token::Boolean(right)) => {
                            self.num_to_bool(left) || right
                        }
                        (Token::Boolean(left), Token::Number(right)) => {
                            left || self.num_to_bool(right)
                        }
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
            }),
            ASTNode::ConditionalOperation {
                condition,
                indent_level,
            } => {
                let condition_val = self.interpret(condition);
                match condition_val {
                    Token::Boolean(true) => {
                        self.indent_level = indent_level.clone();
                        self.pass = false;
                        Token::Boolean(true)
                    }
                    Token::Boolean(false) => {
                        self.indent_level = indent_level.clone();
                        self.pass = true;
                        Token::Boolean(false)
                    }
                    _ => panic!("Error i008: Unexpected value: {:?}", condition_val),
                }
            }
            ASTNode::AlternativeOperation {
                condition: _,
                indent_level,
            } => {
                self.indent_level = indent_level.clone();
                self.pass = true;
                Token::None
            }
        }
    }

    fn num_to_bool(&self, num: i64) -> bool {
        num != 0
    }
}
