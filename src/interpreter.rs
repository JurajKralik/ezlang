use crate::parser::*;
use crate::tokenizer::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interpreter {
    variables: HashMap<String, Token>,
    line_number: usize,
    pass_lock: bool,
    indent_level: usize,
    new_indent_level: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            line_number: 0,
            pass_lock: false,
            indent_level: 0,
            new_indent_level: false,
        }
    }

    pub fn interpret_line(&mut self, node: &ASTNode) -> Token {
        self.line_number += 1;
        let local_indent_level = node.indent_level();

        if self.new_indent_level {
            // Set the indent level to the new indent level
            self.indent_level = local_indent_level;
            self.new_indent_level = false;
        } else if self.pass_lock {
            // Check if the interpreter is in a pass state
            if local_indent_level > self.indent_level {
                return Token::None;
            } else if local_indent_level == self.indent_level {
                match node {
                    ASTNode::AlternativeOperation {
                        condition,
                        indent_level: _,
                    } => {
                        if let Some(elif_condition) = condition {
                            //Elif
                            if self.interpret(elif_condition) == Token::Boolean(true) {
                                self.pass_lock = false;
                                self.new_indent_level = true;
                                return Token::Boolean(true);
                            } else {
                                self.pass_lock = true;
                                return Token::Boolean(false);
                            }
                        } else {
                            //Else
                            self.pass_lock = false;
                            self.new_indent_level = true;
                            return Token::Boolean(true);
                        }
                    }
                    _ => {
                        self.pass_lock = false;
                    }
                }
            }
        } else if local_indent_level > self.indent_level {
            // Off limits
            panic!("Error i010: Unexpected indent level on line {}", self.line_number);
        }
        self.interpret(node)
    }

    fn print_interpret(&mut self, node: &ASTNode) -> String {
        let token = self.interpret(node);
        match token {
            Token::Integer(value) => value.to_string(),
            Token::Float(value) => value.to_string(),
            Token::Boolean(value) => value.to_string(),
            Token::String(value) => value,
            _ => panic!("Error i009: Unexpected token: {:?} on line {}", token, self.line_number),
        }
    }

    fn interpret(&mut self, node: &ASTNode) -> Token {
        match node {
            ASTNode::Integer(value, _indent_level) => Token::Integer(*value),
            ASTNode::Float(value, _indent_level) => Token::Float(*value),
            ASTNode::Boolean(value, _indent_level) => Token::Boolean(*value),
            ASTNode::Identifier(name, _indent_level) => match self.variables.get(name) {
                Some(token) => match token {
                    Token::Integer(value) => Token::Integer(value.clone()),
                    Token::Float(value) => Token::Float(value.clone()),
                    Token::Boolean(value) => Token::Boolean(value.clone()),
                    Token::String(value) => Token::String(value.clone()),
                    _ => panic!("Error i001: Unexpected token: {:?} on line {}", token, self.line_number),
                },
                None => panic!("Error i002: Variable not found: {} on line {}", name, self.line_number),
            },
            ASTNode::String(value, _indent_level) => Token::String(value.clone()),
            ASTNode::OutputOperation {
                value,
                indent_level: _,
            } => {
                let token_value = self.print_interpret(value);
                println!("{}", token_value);
                Token::None
            }
            ASTNode::BinaryOperation {
                left,
                operator,
                right,
                indent_level: _,
            } => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                match operator.clone() {
                    Token::Plus => match (left_val.clone(), right_val.clone()) {
                        (Token::Integer(left_num), Token::Integer(right_num)) => {
                            Token::Integer(left_num + right_num)
                        }
                        (Token::Float(left_num), Token::Float(right_num)) => {
                            Token::Float(left_num + right_num)
                        }
                        (Token::Integer(left_num), Token::Float(right_num)) => {
                            Token::Float(left_num as f64 + right_num)
                        }
                        (Token::Float(left_num), Token::Integer(right_num)) => {
                            Token::Float(left_num + right_num as f64)
                        }
                        (Token::Boolean(left_bool), Token::Boolean(right_bool)) => {
                            Token::Boolean(left_bool || right_bool)
                        }
                        (Token::Integer(_left_num), Token::Boolean(right_bool)) => {
                            Token::Boolean(self.num_to_bool(left_val.clone()) || right_bool)
                        }
                        (Token::Boolean(left_bool), Token::Integer(_right_num)) => {
                            Token::Boolean(left_bool || self.num_to_bool(right_val.clone()))
                        }
                        (Token::String(left_str), Token::String(right_str)) => {
                            Token::String(format!("{}{}", left_str, right_str))
                        }
                        (Token::String(left_str), Token::Integer(right_num)) => {
                            Token::String(format!("{}{}", left_str, right_num))
                        }
                        _ => panic!(
                            "Error i003: Unexpected values {:?}, {:?}, {:?}, on line {}",
                            left_val, operator, right_val, self.line_number
                        ),
                    },
                    Token::Minus => match (left_val.clone(), right_val.clone()) {
                        (Token::Integer(left_num), Token::Integer(right_num)) => {
                            Token::Integer(left_num - right_num)
                        }
                        (Token::Boolean(left_bool), Token::Boolean(right_bool)) => {
                            Token::Boolean(left_bool && !right_bool)
                        }
                        (Token::Boolean(left_bool), Token::Integer(_right_num)) => {
                            Token::Boolean(left_bool && !self.num_to_bool(right_val.clone()))
                        }
                        (Token::Integer(_left_num), Token::Boolean(right_bool)) => {
                            Token::Boolean(self.num_to_bool(left_val.clone()) && !right_bool)
                        }
                        _ => panic!(
                            "Error i003: Unexpected values {:?}, {:?}, {:?}, on line {}",
                            left_val, operator, right_val, self.line_number
                        ),
                    },
                    Token::Asterisk => match (left_val.clone(), right_val.clone()) {
                        (Token::Integer(left_num), Token::Integer(right_num)) => {
                            Token::Integer(left_num * right_num)
                        }
                        (Token::Boolean(left_bool), Token::Boolean(right_bool)) => {
                            Token::Boolean(left_bool && right_bool)
                        }
                        _ => panic!(
                            "Error i003: Unexpected values {:?}, {:?}, {:?}, on line {}",
                            left_val, operator, right_val, self.line_number
                        ),
                    },
                    Token::Slash => {
                        if let (Token::Integer(left_num), Token::Integer(right_num)) =
                            (left_val.clone(), right_val.clone())
                        {
                            Token::Integer(left_num / right_num)
                        } else {
                            panic!(
                                "Error i003: Unexpected values {:?}, {:?}, {:?}, on line {}",
                                left_val, operator, right_val, self.line_number
                            );
                        }
                    }
                    Token::Modulo => {
                        if let (Token::Integer(left_num), Token::Integer(right_num)) =
                            (left_val.clone(), right_val.clone())
                        {
                            Token::Integer(left_num % right_num)
                        } else {
                            panic!(
                                "Error i003: Unexpected values {:?}, {:?}, {:?}, on line {}",
                                left_val, operator, right_val, self.line_number
                            );
                        }
                    }
                    Token::And => match (left_val.clone(), right_val.clone()) {
                        (Token::Boolean(left), Token::Boolean(right)) => {
                            Token::Boolean(left && right)
                        }
                        (Token::Integer(_left), Token::Integer(_right)) => {
                            Token::Boolean(self.num_to_bool(left_val.clone()) && self.num_to_bool(right_val.clone()))
                        }
                        (Token::Integer(_left), Token::Boolean(right)) => {
                            Token::Boolean(self.num_to_bool(left_val.clone()) && right)
                        }
                        (Token::Boolean(left), Token::Integer(_right)) => {
                            Token::Boolean(left && self.num_to_bool(right_val.clone()))
                        }
                        _ => panic!("Error i003: Unexpected values on line {}", self.line_number),
                    },
                    Token::Or => match (left_val.clone(), right_val.clone()) {
                        (Token::Boolean(left), Token::Boolean(right)) => {
                            Token::Boolean(left || right)
                        }
                        (Token::Integer(_left), Token::Integer(_right)) => {
                            Token::Boolean(self.num_to_bool(left_val.clone()) || self.num_to_bool(right_val.clone()))
                        }
                        (Token::Integer(_left), Token::Boolean(right)) => {
                            Token::Boolean(self.num_to_bool(left_val.clone()) || right)
                        }
                        (Token::Boolean(left), Token::Integer(_right)) => {
                            Token::Boolean(left || self.num_to_bool(right_val.clone()))
                        }
                        _ => panic!("Error i003: Unexpected values on line {}", self.line_number),
                    },
                    Token::Comparison(operator) => match operator {
                        Compare::Equal => Token::Boolean(left_val == right_val),
                        Compare::NotEqual => Token::Boolean(left_val != right_val),
                        Compare::LessThan => match (left_val.clone(), right_val.clone()) {
                            (Token::Integer(left), Token::Integer(right)) => {
                                Token::Boolean(left < right)
                            }
                            _ => panic!("Error i003: Unexpected values on line {}", self.line_number),
                        },
                        Compare::LessThanOrEqual => match (left_val.clone(), right_val.clone()) {
                            (Token::Integer(left), Token::Integer(right)) => {
                                Token::Boolean(left <= right)
                            }
                            _ => panic!("Error i003: Unexpected values on line {}", self.line_number),
                        },
                        Compare::GreaterThan => match (left_val.clone(), right_val.clone()) {
                            (Token::Integer(left), Token::Integer(right)) => {
                                Token::Boolean(left > right)
                            }
                            _ => panic!("Error i003: Unexpected values on line {}", self.line_number),
                        },
                        Compare::GreaterThanOrEqual => {
                            match (left_val.clone(), right_val.clone()) {
                                (Token::Integer(left), Token::Integer(right)) => {
                                    Token::Boolean(left >= right)
                                }
                                _ => panic!("Error i003: Unexpected values on line {}", self.line_number),
                            }
                        }
                    },
                    _ => panic!("Error i004: Unexpected operator: {:?} on line {}", operator, self.line_number),
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
                    _ => panic!("Error i005: Unexpected token: {:?}, on line {}", variable, self.line_number),
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
                    match (left_val.clone(), right_val.clone()) {
                        (Token::Boolean(left), Token::Boolean(right)) => left && right,
                        (Token::Integer(_left), Token::Integer(_right)) => {
                            self.num_to_bool(left_val.clone()) && self.num_to_bool(right_val.clone())
                        }
                        (Token::Integer(_left), Token::Boolean(right)) => {
                            self.num_to_bool(left_val.clone()) && right
                        }
                        (Token::Boolean(left), Token::Integer(_right)) => {
                            left && self.num_to_bool(right_val.clone())
                        }
                        _ => panic!("Error i006: Unexpected values on line {}", self.line_number),
                    }
                }
                Token::Or => {
                    let left_val = self.interpret(left);
                    let right_val = self.interpret(right);
                    match (left_val.clone(), right_val.clone()) {
                        (Token::Boolean(left), Token::Boolean(right)) => left || right,
                        (Token::Integer(_left), Token::Integer(_right)) => {
                            self.num_to_bool(left_val.clone()) || self.num_to_bool(right_val.clone())
                        }
                        (Token::Integer(_left), Token::Boolean(right)) => {
                            self.num_to_bool(left_val.clone()) || right
                        }
                        (Token::Boolean(left), Token::Integer(_right)) => {
                            left || self.num_to_bool(right_val.clone())
                        }
                        _ => panic!("Error i006: Unexpected values on line {}", self.line_number),
                    }
                }
                Token::Not => {
                    let right_val = self.interpret(right);
                    match right_val {
                        Token::Boolean(right) => !right,
                        Token::Integer(_right) => !self.num_to_bool(right_val.clone()),
                        _ => panic!("Error i006: Unexpected values on line {}", self.line_number),
                    }
                }
                _ => panic!("Error i007: Unexpected operator: {:?} on line {}", operator, self.line_number),
            }),
            ASTNode::ConditionalOperation {
                condition,
                indent_level,
            } => {
                let condition_val = self.interpret(condition);
                match condition_val {
                    Token::Boolean(true) => {
                        self.indent_level = indent_level.clone();
                        self.pass_lock = false;
                        self.new_indent_level = true;
                        Token::Boolean(true)
                    }
                    Token::Boolean(false) => {
                        self.indent_level = indent_level.clone();
                        self.pass_lock = true;
                        self.new_indent_level = false;
                        Token::Boolean(false)
                    }
                    _ => panic!("Error i008: Unexpected value: {:?} on line {}", condition_val, self.line_number),
                }
            }
            ASTNode::AlternativeOperation {
                condition: _,
                indent_level,
            } => {
                self.indent_level = indent_level.clone();
                self.pass_lock = true;
                Token::None
            }
        }
    }

    fn num_to_bool(&self, token: Token) -> bool {
        match token {
            Token::Integer(num) => num != 0,
            Token::Float(num) => num != 0.0,
            _ => panic!("Error i011: Unexpected token: {:?} on line {}", token, self.line_number)
        }
    }
}
