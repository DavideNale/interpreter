use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    peeked: Option<token::Token>,
}

impl Iterator for Lexer {
    type Item = token::Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.peeked.take() {
            return Some(next);
        }

        let token = self.next_token();
        if token == token::Token::Eof {
            None
        } else {
            Some(token)
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
            peeked: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn peek(&mut self) -> Option<&token::Token> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }

        self.peeked = self.next();
        self.peeked.as_ref()
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let token = match self.ch {
            Some('=') => {
                if let Some(peek) = self.peek_char() {
                    if peek == '=' {
                        self.read_char();
                        return token::Token::Eq;
                    }
                }
                token::Token::Assign
            }
            Some('+') => token::Token::Plus,
            Some('-') => token::Token::Minus,
            Some('!') => {
                if let Some(peek) = self.peek_char() {
                    if peek == '=' {
                        self.read_char();
                        return token::Token::NotEq;
                    }
                }
                token::Token::Bang
            }
            Some('/') => token::Token::Slash,
            Some('*') => token::Token::Asterisk,
            Some('<') => token::Token::Lt,
            Some('>') => token::Token::Gt,
            Some(';') => token::Token::Semicolon,
            Some(',') => token::Token::Comma,
            Some('(') => token::Token::LParen,
            Some(')') => token::Token::RParen,
            Some('{') => token::Token::LBrace,
            Some('}') => token::Token::RBrace,
            None => token::Token::Eof,
            Some(ch) => {
                if is_letter(ch) {
                    let ident = self.read_identifier();
                    token::lookup_ident(&ident)
                } else if ch.is_digit(10) {
                    token::Token::Int(ch.to_string())
                } else {
                    token::Token::Illegal(ch)
                }
            }
        };
        self.read_char();
        token
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

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input[self.read_position..].chars().next()
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.ch {
            if is_letter(ch) {
                self.read_char();
                continue;
            } else {
                break;
            }
        }
        let identifier_str = self.input.get(start..self.position).unwrap();
        String::from(identifier_str)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, Some(' ') | Some('\t') | Some('\n') | Some('\r')) {
            self.read_char()
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
