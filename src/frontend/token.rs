#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Token {
        Token {
            kind,
            line,
            column,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum TokenKind {
    #[default]
    Default,

    Illegal(String),

    // Identifiers + literals
    Ident(String),
    Char(char),
    Int(String),
    Float(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Remainder,

    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    RightArrow,
    
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    While,
    Break,
    Continue,
    Return,
}
