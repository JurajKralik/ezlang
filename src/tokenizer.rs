#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    //TODO Add Logicals, etc.
    None,
    Boolean(bool),
    Number(String),
    Identifier(String),
    Equals,
    Bind,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    OpenParen,
    CloseParen,
    EOL,
    EOF,
    Unknown,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            position: 0
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
            '=' => {
                self.advance();
                Token::Bind
            }
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
        while self.position < self.input.len() && self.current_char().is_digit(10) {
            self.advance();
        }
        Token::Number(self.input[start..self.position].to_string())
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
            "is" => Token::Equals,
            _ => Token::Identifier(token),
        }
    }
}
