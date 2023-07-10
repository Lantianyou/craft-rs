use crate::token::{keyword, Token, TokenType, NULL_CHAR, Literal};
use std::str::FromStr;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token = if self.peek_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::BANG
                };
                self.advance();
                self.add_token(token);
            }
            '=' => {
                let token = if self.peek_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::EQUAL
                };
                self.advance();
                self.add_token(token);
            }
            '<' => {
                let token_type = if self.peek_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::LESS
                };
                self.advance();
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.peek_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::GREATER
                };
                self.advance();
                self.add_token(token_type);
            }
            '/' => {
                if self.peek_match('/') {
                    self.advance();
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            'o' => {
                if self.peek_match('r') {
                    self.advance();
                    self.add_token(TokenType::OR);
                }
            }
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    println!("error: unexpected character {} at line {}", c, self.line);
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let res = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return res;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal<'a>>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("error: unterminated string at line {}", self.line);
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::STRING, Some(Literal::Str(value)));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let val: &str = &self.source[self.start..self.current];
        let num = f64::from_str(val).unwrap();
        self.add_token_literal(TokenType::NUMBER, Some(Literal::Num(num)))
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: &str = &self.source[self.start..self.current];
        let token_type = keyword(text).unwrap_or(TokenType::IDENTIFIER);
        self.add_token(token_type);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return NULL_CHAR;
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return NULL_CHAR;
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn peek_match(&self, expected: char) -> bool {
        return self.peek() == expected;
    }
}
