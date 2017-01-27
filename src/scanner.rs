use token::{Token, TokenType};
use std::str;
use std::fmt;
use std::iter;

pub type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char, usize),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScannerError::UnknownCharacter(c, line) => {
                write!(f, "Unrecognised character {} at line {}", c, line)
            }
        }
    }
}

pub struct Scanner<'a> {
    source: iter::Peekable<str::Chars<'a>>,
    line: usize,
    start: usize,
    offset: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().peekable(),
            line: 1,
            start: 0,
            offset: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> ScannerResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            self.skip_whitespace();

            let token = self.scan_token();
            match token {
                Ok(token) => {
                    if token.ttype == TokenType::EOF {
                        tokens.push(token);
                        break;
                    } else {
                        tokens.push(token);
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if !c.is_whitespace() {
                return;
            }

            if c == '\n' {
                self.line += 1;
            }

            self.source.next();
        }
    }

    fn simple_token(&self, ttype: TokenType) -> Token {
        Token::new(ttype, self.line, String::new())
    }

    fn scan_operator(&mut self, ttype: TokenType, equality_ttype: TokenType) -> Token {
        if self.source.peek() == Some(&'=') {
            // Consume the =
            self.source.next();
            self.simple_token(equality_ttype)
        } else {
            self.simple_token(ttype)
        }
    }

    fn scan_token(&mut self) -> ScannerResult<Token> {
        use token::TokenType::*;
        match self.source.next() {
            Some(',') => Ok(self.simple_token(COMMA)),
            Some('.') => Ok(self.simple_token(DOT)),
            Some('*') => Ok(self.simple_token(STAR)),
            Some('-') => Ok(self.simple_token(MINUS)),
            Some('+') => Ok(self.simple_token(PLUS)),
            Some('/') => Ok(self.simple_token(SLASH)),
            Some(';') => Ok(self.simple_token(SEMICOLON)),
            Some('(') => Ok(self.simple_token(LPAREN)),
            Some(')') => Ok(self.simple_token(RPAREN)),
            Some('{') => Ok(self.simple_token(LBRACE)),
            Some('}') => Ok(self.simple_token(RBRACE)),

            Some('=') => Ok(self.scan_operator(EQUALS, EQUALS_EQUALS)),
            Some('!') => Ok(self.scan_operator(EXCLAM, EXCLAM_EQUALS)),
            Some('<') => Ok(self.scan_operator(LESS, LESS_EQUALS)),
            Some('>') => Ok(self.scan_operator(GREATER, GREATER_EQUALS)),

            Some(c @ _) => Err(ScannerError::UnknownCharacter(c, self.line)),

            None => Ok(self.simple_token(EOF)),
        }
    }
}
