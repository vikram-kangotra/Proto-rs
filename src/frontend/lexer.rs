use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

pub struct Lexer {
    input: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer { 
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.trim().to_string(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn peek_char(&self) -> char {
        self.input.chars().nth(self.current).unwrap_or('\0')
    }

    #[allow(dead_code)]
    pub fn peek_next(&self) -> char {
        self.input.chars().nth(self.current + 1).unwrap_or('\0')
    }

    pub fn advance(&mut self) -> char {
        let c = self.peek_char();
        self.current += 1;
        self.column += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                ' ' | '\t' | '\r' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.advance();
                },
                _ => break,
            }
        }
    }
    
    fn character(&mut self) -> Option<Token> {
        let c = self.advance();
        if self.peek_char() != '\'' {
            return Some(Token::new_with_lexeme(TokenKind::Illegal, self.line, self.column, "Expected closing \'".to_string()));
        }
        self.advance();
        Some(Token::new_with_lexeme(TokenKind::Char, self.line, self.column, c.to_string()))
    }

    fn number(&mut self) -> Option<Token> {
        while self.peek_char().is_digit(10) {
            self.advance();
        }

        if self.peek_char() == '.' {
            self.advance();
            while self.peek_char().is_digit(10) {
                self.advance();
            }
        }

        if self.peek_char() == 'e' || self.peek_char() == 'E' {
            self.advance();
            if self.peek_char() == '+' || self.peek_char() == '-' {
                self.advance();
            }
            while self.peek_char().is_digit(10) {
                self.advance();
            }
        }

        let lexeme = self.input[self.start..self.current].to_string();

        let kind = if lexeme.contains('.') || lexeme.contains('e') || lexeme.contains('E') {
            TokenKind::Float
        } else {
            TokenKind::Int
        };

        Some(Token::new_with_lexeme(kind, self.line, self.column, lexeme))
    }

    fn identifier(&mut self) -> Option<Token> {
        while self.peek_char().is_alphabetic() || self.peek_char().is_digit(10) || self.peek_char() == '_' {
            self.advance();
        }

        let lexeme = self.input.chars()
                        .skip(self.start)
                        .take(self.current - self.start)
                        .collect::<String>();

        let kind = match lexeme.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "let" => TokenKind::Let,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            _ => TokenKind::Ident,
        };

        Some(Token::new_with_lexeme(kind, self.line, self.column, lexeme))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current >= self.input.len() {
            return None;
        }

        self.skip_whitespace();

        self.start = self.current;

        match self.advance() {
            '-' => Some(Token::new(TokenKind::Minus, self.line, self.column)),
            '+' => Some(Token::new(TokenKind::Plus, self.line, self.column)),
            '*' => Some(Token::new(TokenKind::Asterisk, self.line, self.column)),
            '/' => {
                if self.peek_char() == '/' {
                    while self.peek_char() != '\n' && self.current < self.input.len() {
                        self.advance();
                    }
                    self.line += 1;
                    self.column = 1;
                    self.next()
                } else if self.peek_char() == '*' {
                    self.advance();
                    while self.current < self.input.len() && (self.peek_char() != '*' || self.peek_next() != '/') {
                        self.advance();
                        self.column += 1;
                        self.skip_whitespace();
                    }
                    self.advance();
                    self.advance();
                    self.next()
                } else {
                    Some(Token::new(TokenKind::Slash, self.line, self.column))
                }
            }
            '%' => Some(Token::new(TokenKind::Remainder, self.line, self.column)),
            '(' => Some(Token::new(TokenKind::LeftParen, self.line, self.column)),
            ')' => Some(Token::new(TokenKind::RightParen, self.line, self.column)),
            '{' => Some(Token::new(TokenKind::LeftBrace, self.line, self.column)),
            '}' => Some(Token::new(TokenKind::RightBrace, self.line, self.column)),
            ',' => Some(Token::new(TokenKind::Comma, self.line, self.column)),
            ';' => Some(Token::new(TokenKind::Semicolon, self.line, self.column)),
            '<' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::LessEqual, self.line, self.column))
                } else {
                    Some(Token::new(TokenKind::Less, self.line, self.column))
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::GreaterEqual, self.line, self.column))
                } else {
                    Some(Token::new(TokenKind::Greater, self.line, self.column))
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::Equal, self.line, self.column))
                } else {
                    Some(Token::new(TokenKind::Assign, self.line, self.column))
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::NotEqual, self.line, self.column))
                } else {
                    Some(Token::new(TokenKind::Bang, self.line, self.column))
                }
            },
            '\'' => self.character(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => {
                self.advance();
                Some(Token::new(TokenKind::Illegal, self.line, self.column))
            },
        }
    }
}
