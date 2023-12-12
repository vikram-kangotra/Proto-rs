use crate::frontend::token::Token;
use crate::frontend::token::TokenKind;

pub struct Lexer {
    input: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer { 
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_owned(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn peek_char(&self) -> char {
        self.input.chars().nth(self.current).unwrap_or('\0')
    }

    pub fn peek_next(&self) -> char {
        self.input.chars().nth(self.current + 1).unwrap_or('\0')
    }

    pub fn advance(&mut self) -> char {
        let c = self.peek_char();
        self.current += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                ' ' | '\t' | '\r' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => break,
            }
        }
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

        Some(Token::new_with_lexeme(kind, lexeme))
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
            _ => TokenKind::Ident,
        };

        Some(Token::new_with_lexeme(kind, lexeme))
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
            '-' => Some(Token::new(TokenKind::Minus)),
            '+' => Some(Token::new(TokenKind::Plus)),
            '*' => Some(Token::new(TokenKind::Asterisk)),
            '/' => Some(Token::new(TokenKind::Slash)),
            '(' => Some(Token::new(TokenKind::LParen)),
            ')' => Some(Token::new(TokenKind::RParen)),
            '{' => Some(Token::new(TokenKind::LBrace)),
            '}' => Some(Token::new(TokenKind::RBrace)),
            ',' => Some(Token::new(TokenKind::Comma)),
            ';' => Some(Token::new(TokenKind::Semicolon)),
            '<' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::LessEqual))
                } else {
                    Some(Token::new(TokenKind::Less))
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::GreaterEqual))
                } else {
                    Some(Token::new(TokenKind::Greater))
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::Equal))
                } else {
                    Some(Token::new(TokenKind::Assign))
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.advance();
                    Some(Token::new(TokenKind::NotEqual))
                } else {
                    Some(Token::new(TokenKind::Bang))
                }
            },
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => {
                self.advance();
                Some(Token::new(TokenKind::Illegal))
            },
        }
    }
}
