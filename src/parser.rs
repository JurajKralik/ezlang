use crate::tokenizer::*;


#[derive(Debug)]
pub enum ASTNode {
    Number(i64, usize),
    Identifier(String, usize),
    Boolean(bool, usize),
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
        indent_level: usize
    },
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        let mut parser = Parser { tokenizer, current_token: Token::EOF };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }

    pub fn parse(&mut self) -> ASTNode {
        if self.tokenizer.peek_token() == Token::Bind {
            let variable = self.current_token.clone();
            self.advance();
            self.advance();
            let node = self.expression();
            ASTNode::BindingOperation {
                variable,
                value: Box::new(node),
                indent_level: self.tokenizer.indent_level,}
        } else if self.current_token == Token::If {
            self.advance();
            let node = self.expression();
            self.expect(Token::Colon);
            ASTNode::ConditionalOperation {
                condition: Box::new(node),
                indent_level: self.tokenizer.indent_level,
            }
        } else {
            self.expression()
        }
    }

    fn expression(&mut self) -> ASTNode {
        let mut node = self.term();

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.term()),
                indent_level: self.tokenizer.indent_level,
            };
        }

        node
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();

        while self.current_token == Token::Asterisk || self.current_token == Token::Slash || self.current_token == Token::Modulo || self.current_token == Token::And || self.current_token == Token::Or {
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.factor()),
                indent_level: self.tokenizer.indent_level,
            };
        }

        node
    }

    fn factor(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Number(value ) => {
                let number = value.clone();
                self.advance();
                ASTNode::Number(number, self.tokenizer.indent_level)
            }
            Token::Identifier(name) => {
                let identifier = name.clone();
                self.advance();
                ASTNode::Identifier(identifier, self.tokenizer.indent_level)
            }
            Token::OpenParen => {
                self.advance();
                let node = self.expression();
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
                let node = self.factor();
                ASTNode::LogicalOperation {
                    left: Box::new(ASTNode::Boolean(false, self.tokenizer.indent_level)),
                    operator: Token::Not,
                    right: Box::new(node),
                    indent_level: self.tokenizer.indent_level,
                }
            } 
            _ => panic!("Error p001: Unexpected token: {:?}", self.current_token),
        }
    }

    fn expect(&mut self, expected_token: Token) {
        if self.current_token == expected_token {
            self.advance();
        } else {
            panic!("Error p002: Expected token: {:?}, but found: {:?}", expected_token, self.current_token);
        }
    }
}
