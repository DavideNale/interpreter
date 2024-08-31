use crate::ast::Ast;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    pub fn parse_program(&mut self) {
        let mut ast = Ast::new();

        while let Some(tok) = self.lexer.next() {
            match tok {
                Token::Eof => break,
                _ => {
                    println!("{}\n", tok)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_let() {
        let s = "let x = 5;";
        Parser::new(s.to_string()).parse_program()
    }
}
