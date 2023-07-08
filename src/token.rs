use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    IDENTIFIER,
    STRING,
    NUMBER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

pub const NULL_CHAR: char = '\0';

pub fn keyword(s: &str) -> Option<TokenType> {
    KEYWORDS.get(s).map(|t| *t)
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", TokenType::AND);
        map.insert("class", TokenType::CLASS);
        map.insert("else", TokenType::ELSE);
        map.insert("false", TokenType::FALSE);
        map.insert("for", TokenType::FOR);
        map.insert("fun", TokenType::FUN);
        map.insert("if", TokenType::IF);
        map.insert("nil", TokenType::NIL);
        map.insert("or", TokenType::OR);
        map.insert("print", TokenType::PRINT);
        map.insert("return", TokenType::RETURN);
        map.insert("super", TokenType::SUPER);
        map.insert("this", TokenType::THIS);
        map.insert("true", TokenType::TRUE);
        map.insert("var", TokenType::VAR);
        map.insert("while", TokenType::WHILE);
        map
    };
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::DOT => write!(f, "DOT"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::STAR => write!(f, "STAR"),
            TokenType::BANG => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::EQUAL => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::GREATER => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::LESS => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::AND => write!(f, "AND"),
            TokenType::CLASS => write!(f, "CLASS"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::FUN => write!(f, "FUN"),
            TokenType::FOR => write!(f, "FOR"),
            TokenType::IF => write!(f, "IF"),
            TokenType::NIL => write!(f, "NIL"),
            TokenType::OR => write!(f, "OR"),
            TokenType::PRINT => write!(f, "PRINT"),
            TokenType::RETURN => write!(f, "RETURN"),
            TokenType::SUPER => write!(f, "SUPER"),
            TokenType::THIS => write!(f, "THIS"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::VAR => write!(f, "VAR"),
            TokenType::WHILE => write!(f, "WHILE"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
pub enum Literal<'a> {
    Str(&'a str),
    Num(f64),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: u32,
    literal: Option<Literal<'a>>,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<Literal<'a>>,
        line: u32,
    ) -> Token<'a> {
        Token {
            token_type,
            literal,
            line,
            lexeme,
        }
    }
}
