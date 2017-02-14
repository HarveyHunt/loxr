use token::{Token, TokenType, Literal};
use std::str;
use std::fmt;
use std::iter;

pub type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char, usize),
    UnterminatedString(usize),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScannerError::UnknownCharacter(c, line) => {
                write!(f, "Unrecognised character {} at line {}", c, line)
            }
            ScannerError::UnterminatedString(line) => {
                write!(f, "Unterminated string at line {}", line)
            }
        }
    }
}

impl ScannerError {
    pub fn line(&self) -> usize {
        match *self {
            ScannerError::UnknownCharacter(_, line) => line,
            ScannerError::UnterminatedString(line) => line,
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

    fn parse_identifier(&mut self, c: char) -> ScannerResult<Token> {
        let mut string = String::new();
        string.push(c);

        while let Some(&c) = self.source.peek() {
            if !c.is_alphabetic() && c != '_' && !c.is_digit(10) {
                break;
            }
            string.push(c);
            self.source.next();
        }

        // TODO: Check if the identifier is a keyword.
        return Ok(Token::new(TokenType::IDENT, self.line, Some(string), None));
    }

    fn parse_string(&mut self) -> ScannerResult<Token> {
        let mut string = String::new();
        let line = self.line;

        while let Some(&c) = self.source.peek() {
            if c == '"' {
                // Skip closing "
                self.source.next();
                return Ok(Token::new(TokenType::STR,
                                     self.line,
                                     Some(string.clone()),
                                     Some(Literal::Str(string))));
            } else if c == '\n' {
                self.line += 1;
            }
            string.push(self.source.next().unwrap());
        }

        return Err(ScannerError::UnterminatedString(line));
    }

    fn parse_number(&mut self, c: char) -> ScannerResult<Token> {
        let mut string = String::new();
        string.push(c);

        while let Some(&c) = self.source.peek() {
            if c == '.' {
                string.push(c);
            } else if c.is_digit(10) {
                string.push(c);
            } else {
                break;
            }
            self.source.next();
        }
        let float: f64 = string.parse().unwrap();
        return Ok(Token::new(TokenType::NUMBER,
                             self.line,
                             Some(string),
                             Some(Literal::Float(float))));
    }

    fn simple_token(&self, ttype: TokenType) -> Token {
        Token::new(ttype, self.line, None, None)
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

            '"' => self.parse_string(),
            c @ _ => {
                if c.is_digit(10) {
                    self.parse_number(c)
                } else if c.is_alphabetic() || c == '_' || c.is_digit(10) {
                    self.parse_identifier(c)
                } else {
                    Err(ScannerError::UnknownCharacter(c, self.line))
                }
            }
        }
    }
}
