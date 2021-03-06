#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    COMMA,
    DOT,
    STAR,
    MINUS,
    PLUS,
    EQUALS,
    SLASH,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    EQUALS_EQUALS,
    EXCLAM,
    EXCLAM_EQUALS,
    LESS,
    LESS_EQUALS,
    GREATER,
    GREATER_EQUALS,

    NUMBER,
    IDENT,
    STR,

    AND,
    OR,
    IF,
    ELSE,
    VAR,
    FOR,
    WHILE,

    FUN,
    CLASS,
    SUPER,
    THIS,
    RETURN,

    TRUE,
    FALSE,
    NIL,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Float(f64),
}

#[derive(Clone, Debug)]
pub struct Token {
    // TODO: Make these const.
    pub ttype: TokenType,
    line: usize,
    lexeme: Option<String>,
    literal: Option<Literal>,
}

impl Token {
    pub fn new(ttype: TokenType,
               line: usize,
               lexeme: Option<String>,
               literal: Option<Literal>)
               -> Token {
        Token {
            ttype: ttype,
            line: line,
            lexeme: lexeme,
            literal: literal,
        }
    }
}
