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

impl ScannerError {
    pub fn line(&self) -> usize {
        match *self {
            ScannerError::UnknownCharacter(_, line) => line,
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
            let c = self.source.next();
            match c {
                Some(c) => {
                    // TODO: This feels like a hacky way of stripping comments out...
                    if !self.skip_comments(c) {
                        match self.scan_token(c) {
                            Ok(token) => tokens.push(token),
                            Err(err) => return Err(err),
                        }
                    }
                }
                None => {
                    tokens.push(self.simple_token(TokenType::EOF));
                    break;
                }
            }
        }

        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if !c.is_whitespace() {
                return;
            } else if c == '\n' {
                self.line += 1;
            }

            self.source.next();
        }
    }

    fn skip_comments(&mut self, c: char) -> bool {
        if c == '/' && self.source.peek() == Some(&'/') {
            while let Some(&c) = self.source.peek() {
                self.source.next();
                if c == '\n' {
                    self.line += 1;
                    return true;
                }
            }
        }
        return false;
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

    fn scan_token(&mut self, c: char) -> ScannerResult<Token> {
        use token::TokenType::*;
        match c {
            ',' => Ok(self.simple_token(COMMA)),
            '.' => Ok(self.simple_token(DOT)),
            '*' => Ok(self.simple_token(STAR)),
            '-' => Ok(self.simple_token(MINUS)),
            '+' => Ok(self.simple_token(PLUS)),
            ';' => Ok(self.simple_token(SEMICOLON)),
            '(' => Ok(self.simple_token(LPAREN)),
            ')' => Ok(self.simple_token(RPAREN)),
            '{' => Ok(self.simple_token(LBRACE)),
            '}' => Ok(self.simple_token(RBRACE)),

            '/' => Ok(self.simple_token(SLASH)),

            '=' => Ok(self.scan_operator(EQUALS, EQUALS_EQUALS)),
            '!' => Ok(self.scan_operator(EXCLAM, EXCLAM_EQUALS)),
            '<' => Ok(self.scan_operator(LESS, LESS_EQUALS)),
            '>' => Ok(self.scan_operator(GREATER, GREATER_EQUALS)),

            c @ _ => Err(ScannerError::UnknownCharacter(c, self.line)),
        }
    }
}
