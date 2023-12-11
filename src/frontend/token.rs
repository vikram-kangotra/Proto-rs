#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: None,
        }
    }

    pub fn new_with_literal(kind: TokenKind, lexeme: String) -> Token {
        Token {
            kind,
            lexeme: Some(lexeme),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum TokenKind {
    #[default]
    Illegal,
    EOF,

    // Identifiers + literals
    Ident,
    Int,
    Float,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
