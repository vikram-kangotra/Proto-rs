#[derive(Debug, Default, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Token {
        Token {
            kind,
            lexeme: None,
            line,
            column,
        }
    }

    pub fn new_with_lexeme(kind: TokenKind, line: usize, column: usize, lexeme: String) -> Token {
        Token {
            kind,
            lexeme: Some(lexeme),
            line,
            column,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum TokenKind {
    #[default]
    Illegal,

    // Identifiers + literals
    Ident,
    Char,
    Int,
    Float,

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
    Semicolon,

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
    Return,
}
