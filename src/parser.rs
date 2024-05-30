use crate::tokenizer::*;


#[derive(Debug)]
pub enum ASTNode {
    Number(i64),
    Identifier(String),
    BinaryOperation {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
    },
    BindingOperation {
        variable: Token,
        value: Box<ASTNode>,
    }
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
                value: Box::new(node),}
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
            };
        }

        node
    }

    fn term(&mut self) -> ASTNode {
        let mut node = self.factor();

        while self.current_token == Token::Asterisk || self.current_token == Token::Slash || self.current_token == Token::Modulo {
            let operator = self.current_token.clone();
            self.advance();
            node = ASTNode::BinaryOperation {
                left: Box::new(node),
                operator,
                right: Box::new(self.factor()),
            };
        }

        node
    }

    fn factor(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Number(value) => {
                let number = value.parse::<i64>().unwrap();
                self.advance();
                ASTNode::Number(number)
            }
            Token::Identifier(name) => {
                let identifier = name.clone();
                self.advance();
                ASTNode::Identifier(identifier)
            }
            Token::OpenParen => {
                self.advance();
                let node = self.expression();
                self.expect(Token::CloseParen);
                node
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn expect(&mut self, expected_token: Token) {
        if self.current_token == expected_token {
            self.advance();
        } else {
            panic!("Expected token: {:?}, but found: {:?}", expected_token, self.current_token);
        }
    }
}
