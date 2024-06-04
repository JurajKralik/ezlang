use std::fmt;

#[derive(Debug)]
pub enum InterpreterError {
    LexerError(String),
    ParserError(String),
    RuntimeError(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::LexerError(msg) => write!(f, "Lexer Error: {}", msg),
            InterpreterError::ParserError(msg) => write!(f, "Parser Error: {}", msg),
            InterpreterError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl std::error::Error for InterpreterError {}
