use core::panic;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    //TODO Add Logicals, etc.
    None,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Comparison(Compare),
    Bind,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    And,
    Or,
    Not,
    OpenParen,
    CloseParen,
    If,
    Else,
    ElseIf,
    For,
    Print,
    Colon,
    EOF,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Compare {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
    pub indent_level: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut indent_level =  0;
        for c in input.chars() {
            if c == ' ' {
                indent_level += 1;
            } else {
                break;
            }
        }

        Tokenizer {
            input,
            position: 0,
            indent_level: indent_level,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.current_char();

        match current_char {
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '=' => self.equals(),
            '"' => self.string(),
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                Token::Minus
            }
            '*' => {
                self.advance();
                Token::Asterisk
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '%' => {
                self.advance();
                Token::Modulo
            }
            '(' => {
                self.advance();
                Token::OpenParen
            }
            ')' => {
                self.advance();
                Token::CloseParen
            }
            '&' => {
                self.advance();
                Token::And
            }
            '|' => {
                self.advance();
                Token::Or
            }
            '!' => self.exclamation(),
                        ':' => {
                self.advance();
                Token::Colon
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Token::Comparison(Compare::LessThanOrEqual)
                } else {
                    Token::Comparison(Compare::LessThan)
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Token::Comparison(Compare::GreaterThanOrEqual)
                } else {
                    Token::Comparison(Compare::GreaterThan)
                }
            }
            _ => {
                self.advance();
                Token::Unknown
            }
        }
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap()
    }

    pub fn peek_token(&mut self) -> Token {
        let position = self.position.clone();
        self.skip_whitespace();
        let next_token = self.next_token();
        self.position = position;
        next_token
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn number(&mut self) -> Token {
        let start = self.position;
        let mut point_used = false;
        while self.position < self.input.len() && self.current_char().is_digit(10) || self.current_char() == '.'{
            if self.current_char() == '.' {
                if point_used {
                    panic!("Error t001: Too many points in number literal");
                }
                point_used = true;
            }
            self.advance();
        }
        let number_str = &self.input[start..self.position];
        if point_used {
            let number = number_str.parse::<f64>().unwrap();
            return Token::Float(number);
        } else {
            let number = number_str.parse::<i64>().unwrap();
            return Token::Integer(number);
        }
    }

    fn string(&mut self) -> Token {
        self.advance();
        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '"' {
            self.advance();
        }
        let string = self.input[start..self.position].to_string();
        self.advance();
        Token::String(string)
    }

    fn identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.current_char().is_alphanumeric() {
            self.advance();
        }

        let token = self.input[start..self.position].to_string();

        match token.as_str() {
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "is" => Token::Comparison(Compare::Equal),
            "if" => Token::If,
            "else" => Token::Else,
            "elseif" => Token::ElseIf,
            "print" => Token::Print,
            "for" => Token::For,
            _ => Token::Identifier(token),
        }
    }
    fn equals(&mut self) -> Token {
        self.advance();
        if self.current_char() == '=' {
            self.advance();
            Token::Comparison(Compare::Equal)
        } else {
            Token::Bind
        }
    }

    fn exclamation(&mut self) -> Token {
        self.advance();
        if self.current_char() == '=' {
            self.advance();
            Token::Comparison(Compare::NotEqual)
        } else {
            Token::Not
        }
    }
}
