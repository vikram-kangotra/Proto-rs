#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token {
            kind,
            literal: None,
        }
    }

    pub fn new_with_literal(kind: TokenKind, literal: String) -> Token {
        Token {
            kind,
            literal: Some(literal),
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
