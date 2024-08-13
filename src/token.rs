use phf::phf_map;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // Identifiers + literals
    Ident(String),
    Int(String),
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
    // Special tokens
    Illegal(char),
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(s) | Token::Int(s) => write!(f, "{}", s),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Eq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Illegal(c) => write!(f, "Illegal({})", c),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "fn" => Token::Function,
    "let" => Token::Let,
    "true" => Token::True,
    "false" => Token::False,
    "if" => Token::If,
    "else" => Token::Else,
    "return" => Token::Return,
};

pub fn lookup_ident(ident: &str) -> Token {
    KEYWORDS
        .get(ident)
        .cloned()
        .unwrap_or_else(|| Token::Ident(ident.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_display() {
        assert_eq!(Token::Assign.to_string(), "=");
        assert_eq!(Token::Ident("foo".to_string()).to_string(), "foo");
        assert_eq!(Token::Int("42".to_string()).to_string(), "42");
        assert_eq!(Token::Illegal('?').to_string(), "Illegal(?)");
    }

    #[test]
    fn test_lookup_ident() {
        assert_eq!(lookup_ident("fn"), Token::Function);
        assert_eq!(lookup_ident("foo"), Token::Ident("foo".to_string()));
    }
}
