use crate::tokenizer::*;

#[derive(Debug)]
pub enum ASTNode {
    Integer(i64, usize),
    Float(f64, usize),
    Identifier(String, usize),
    Boolean(bool, usize),
    String(String, usize),
    BinaryOperation {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
        indent_level: usize,
    },
    BindingOperation {
        variable: Token,
        value: Box<ASTNode>,
        indent_level: usize,
    },
    LogicalOperation {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
        indent_level: usize,
    },
    ConditionalOperation {
        condition: Box<ASTNode>,
        indent_level: usize,
    },
    AlternativeOperation {
        condition: Option<Box<ASTNode>>,
        indent_level: usize,
    },
    OutputOperation {
        value: Box<ASTNode>,
        indent_level: usize,
    },
}

impl ASTNode {
    pub fn indent_level(&self) -> usize {
        match self {
            ASTNode::Integer(_, level) => *level,
            ASTNode::Float(_, level) => *level,
            ASTNode::Identifier(_, level) => *level,
            ASTNode::Boolean(_, level) => *level,
            ASTNode::String(_, level) => *level,
            ASTNode::BinaryOperation { indent_level, .. } => *indent_level,
            ASTNode::BindingOperation { indent_level, .. } => *indent_level,
            ASTNode::LogicalOperation { indent_level, .. } => *indent_level,
            ASTNode::ConditionalOperation { indent_level, .. } => *indent_level,
            ASTNode::AlternativeOperation { indent_level, .. } => *indent_level,
            ASTNode::OutputOperation { indent_level, .. } => *indent_level,
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        let mut parser = Parser {
            tokenizer,
            current_token: Token::EOF,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }

    fn expect(&mut self, expected_token: Token) {
        if self.current_token == expected_token {
            self.advance();
        } else {
            panic!(
                "Error p002: Expected token: {:?}, but found: {:?}",
                expected_token, self.current_token
            );
        }
    }

    pub fn parse(&mut self) -> ASTNode {
        // Binding
        if self.tokenizer.peek_token() == Token::Bind {
            self.parse_binding()
        } else {
            self.parse_condition()
        }
    }

    fn parse_binding(&mut self) -> ASTNode {
        let variable = self.current_token.clone();
        self.advance();
        self.advance();
        let node = self.parse_or();
        ASTNode::BindingOperation {
            variable,
            value: Box::new(node),
            indent_level: self.tokenizer.indent_level,
        }
    }

    fn parse_condition(&mut self) -> ASTNode {
        match self.current_token {
            // Conditional
            Token::If => {
                self.advance();
                let node = self.parse_or();
                self.expect(Token::Colon);
                ASTNode::ConditionalOperation {
                    condition: Box::new(node),
                    indent_level: self.tokenizer.indent_level,
                }
            }
            Token::Else => ASTNode::AlternativeOperation {
                condition: None,
                indent_level: self.tokenizer.indent_level,
            },
            Token::ElseIf => {
                self.advance();
                let node = self.parse_or();
                self.expect(Token::Colon);
                ASTNode::AlternativeOperation {
                    condition: Some(Box::new(node)),
                    indent_level: self.tokenizer.indent_level,
                }
            }
            _ => self.parse_print(),
        }
    }

    fn parse_print(&mut self) -> ASTNode {
        match self.current_token {
            //Print
            Token::Print => {
                self.advance();
                self.expect(Token::OpenParen);
                let node = self.parse_or();
                self.expect(Token::CloseParen);
                ASTNode::OutputOperation {
                    value: Box::new(node),
                    indent_level: self.tokenizer.indent_level,
                }
            }
            _ => self.parse_or(),
        }
    }

    fn parse_or(&mut self) -> ASTNode {
        let mut node = self.parse_and();

        while self.current_token == Token::Or{
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.parse_and()),
                indent_level: self.tokenizer.indent_level,
            };
        }
        node
    }

    fn parse_and(&mut self) -> ASTNode {
        let mut node = self.parse_comparison();

        while self.current_token == Token::And{
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.parse_comparison()),
                indent_level: self.tokenizer.indent_level,
            };
        }
        node
    }

    fn parse_comparison(&mut self) -> ASTNode {
        let mut node = self.parse_addition();

        while self.current_token == Token::Comparison(Compare::Equal)
        || self.current_token == Token::Comparison(Compare::NotEqual)
        || self.current_token == Token::Comparison(Compare::GreaterThan)
        || self.current_token == Token::Comparison(Compare::LessThan)
        || self.current_token == Token::Comparison(Compare::GreaterThanOrEqual)
        || self.current_token == Token::Comparison(Compare::LessThanOrEqual){
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.parse_addition()),
                indent_level: self.tokenizer.indent_level,
            };
        }
        node
    }

    fn parse_addition(&mut self) -> ASTNode {
        let mut node = self.parse_multiplication();

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.parse_multiplication()),
                indent_level: self.tokenizer.indent_level,
            };
        }
        node
    }

    fn parse_multiplication(&mut self) -> ASTNode {
        let mut node = self.parse_primary();

        while self.current_token == Token::Asterisk
            || self.current_token == Token::Slash
            || self.current_token == Token::Modulo
        {
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.parse_primary()),
                indent_level: self.tokenizer.indent_level,
            };
        }
        node
    }

    fn parse_primary(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Integer(value) => {
                let number = value.clone();
                self.advance();
                ASTNode::Integer(number, self.tokenizer.indent_level)
            }
            Token::Float(value) => {
                let number = value.clone();
                self.advance();
                ASTNode::Float(number, self.tokenizer.indent_level)
            }
            Token::Identifier(name) => {
                let identifier = name.clone();
                self.advance();
                ASTNode::Identifier(identifier, self.tokenizer.indent_level)
            }
            Token::OpenParen => {
                self.advance();
                let node = self.parse_or();
                self.expect(Token::CloseParen);
                node
            }
            Token::Boolean(value) => {
                let bool = value.clone();
                self.advance();
                ASTNode::Boolean(bool, self.tokenizer.indent_level)
            }
            Token::Not => {
                self.advance();
                let node = self.parse_primary();
                ASTNode::LogicalOperation {
                    left: Box::new(ASTNode::Boolean(false, self.tokenizer.indent_level)),
                    operator: Token::Not,
                    right: Box::new(node),
                    indent_level: self.tokenizer.indent_level,
                }
            } 
            Token::String(value) => {
                let string = value.clone();
                self.advance();
                ASTNode::String(string, self.tokenizer.indent_level)
            }
            _ => panic!("Error p001: Unexpected token: {:?}", self.current_token),
        }
    }
}
