use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            self.ch = self.input[self.read_position..].chars().next();
        }
        self.position = self.read_position;
        self.read_position += self.ch.map_or(0, |c| c.len_utf8());
    }

    pub fn next_token(&mut self) -> token::Token {
        let token = match self.ch {
            Some('=') => token::Token::new(token::TokenType::Assign, "="),
            Some(';') => token::Token::new(token::TokenType::Semicolon, ";"),
            Some('(') => token::Token::new(token::TokenType::LParen, "("),
            Some(')') => token::Token::new(token::TokenType::RParen, ")"),
            Some(',') => token::Token::new(token::TokenType::Comma, ","),
            Some('+') => token::Token::new(token::TokenType::Plus, "+"),
            Some('{') => token::Token::new(token::TokenType::LBrace, "{"),
            Some('}') => token::Token::new(token::TokenType::RBrace, "}"),
            _ => token::Token::new(token::TokenType::Eof, ""),
            // _ => {
            //     if let Some(ch) = self.ch {
            //         if ch.is_alphabetic() || ch == "_" {}
            //     }
            // }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> token::Token {
        let start = self.position;
        while let Some(ch) = self.ch {
            if ch.is_alphabetic() || ch == '_' {
                self.read_char();
                continue;
            } else {
                break;
            }
        }
        let identifier_str = self.input.get(start..self.position).unwrap();
        let identifier = String::from(identifier_str);
        token::Token {
            token_type: token::TokenType::Ident(identifier.clone()),
            literal: identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*; // Import the necessary types and functions

        #[test]
        fn test_next_token() {
            let input = "=+(){},;";

            let tests = vec![
                (token::TokenType::Assign, "="),
                (token::TokenType::Plus, "+"),
                (token::TokenType::LParen, "("),
                (token::TokenType::RParen, ")"),
                (token::TokenType::LBrace, "{"),
                (token::TokenType::RBrace, "}"),
                (token::TokenType::Comma, ","),
                (token::TokenType::Semicolon, ";"),
                (token::TokenType::Eof, ""),
            ];

            let mut lexer = Lexer::new(input.to_string());

            for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
                let tok = lexer.next_token();

                assert_eq!(
                    tok.token_type, *expected_type,
                    "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                    i, expected_type, tok.token_type
                );

                assert_eq!(
                    tok.literal, *expected_literal,
                    "tests[{}] - literal wrong. expected={}, got={}",
                    i, expected_literal, tok.literal
                );
            }
        }
    }
}
