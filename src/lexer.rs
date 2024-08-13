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
                    let token = self.read_identifier();
                    token
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

    fn read_identifier(&mut self) -> token::Token {
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
        let ident = String::from(identifier_str);
        token::Token::Ident(ident)
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
