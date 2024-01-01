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

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub enum TokenKind {
    #[default]
    Default,

    Illegal(String),

    // Identifiers + literals
    Ident(String),
    Char(char),
    Int(String),
    Float(String),
    String(String),

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

    // Types
    U8,
    U16,
    U32,
    U64,

    I8,
    I16,
    I32,
    I64,

    F32,
    F64,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    RightArrow,
    
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

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
